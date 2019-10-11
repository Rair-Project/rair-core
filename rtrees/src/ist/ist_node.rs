/**
 * ist_node.rs: Augmented Interval Search Tree node implementation.
 *  Copyright (C) 2019  Oddcoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 **/
use super::interval::*;
use rbtree::RBTree;

// Decision function:
// In case of the accept function First argument is the node key, second argument is search key.
// In case of the recurse function First argument is the node augmented data and second argument is the search key.
type Decision<K> = dyn Fn(&Interval<K>, &Interval<K>) -> bool;

pub(super) trait ISTHelpers<K: Ord + Copy, V> {
    fn generic_search(&self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<&V>;
    fn generic_search_mut(&mut self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<&mut V>;
    fn generic_key_search(&self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<Interval<K>>;
    fn generic_delete(&mut self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<V>;
}
impl<K: Ord + Copy, V> ISTHelpers<K, V> for RBTree<Interval<K>, Interval<K>, Vec<V>> {
    fn generic_search(&self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<&V> {
        let mut result = if self.left_ref().is_node() && recurse(&self.left_ref().aug_data(), &int) {
            self.left_ref().generic_search(int, recurse, accept)
        } else {
            Vec::new()
        };
        if accept(&self.key(), &int) {
            result.extend(self.data_ref().iter());
        }
        if self.right_ref().is_node() && recurse(&self.right_ref().aug_data(), &int) {
            result.extend(self.right_ref().generic_search(int, recurse, accept));
        }
        return result;
    }
    fn generic_search_mut(&mut self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<&mut V> {
        let key = self.key();
        let (left, right, data) = self.mut_me();
        let mut result = if left.is_node() && recurse(&left.aug_data(), &int) {
            left.generic_search_mut(int, recurse, accept)
        } else {
            Vec::new()
        };
        if accept(&key, &int) {
            result.extend(data.iter_mut());
        }
        if right.is_node() && recurse(&right.aug_data(), &int) {
            result.extend(right.generic_search_mut(int, recurse, accept));
        }
        return result;
    }
    fn generic_key_search(&self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<Interval<K>> {
        let mut keys = if self.left_ref().is_node() && recurse(&self.left_ref().aug_data(), &int) {
            self.left_ref().generic_key_search(int, recurse, accept)
        } else {
            Vec::new()
        };
        if accept(&self.key(), &int) {
            keys.push(self.key());
        }
        if self.right_ref().is_node() && recurse(&self.right_ref().aug_data(), &int) {
            keys.extend(self.right_ref().generic_key_search(int, recurse, accept));
        }
        return keys;
    }
    fn generic_delete(&mut self, int: Interval<K>, recurse: &Decision<K>, accept: &Decision<K>) -> Vec<V> {
        let delete_keys = self.generic_key_search(int, recurse, accept);
        let mut result = Vec::with_capacity(delete_keys.len());
        for key in delete_keys {
            // we can safely unwrap because we already queried the keys!
            result.extend(self.delete(key).unwrap());
        }
        return result;
    }
}
