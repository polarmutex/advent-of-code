use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn dijkstra<T, FK, K, FF, FE, E>(
    initial: impl IntoIterator<Item = T>,
    mut key: FK,
    mut found: FF,
    mut edges: FE,
) -> Option<(usize, T)>
where
    FK: FnMut(&T) -> K,
    K: Hash + Eq,
    FF: FnMut(&T) -> bool,
    FE: FnMut(T) -> E,
    E: IntoIterator<Item = T>,
{
    struct Entry<T> {
        cost: usize,
        item: T,
    }
    impl<T> Ord for Entry<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            let order = self.cost.cmp(&other.cost);
            if order == Ordering::Less {
                Ordering::Greater
            } else if order == Ordering::Greater {
                Ordering::Less
            } else {
                order
            }
        }
    }
    impl<T> PartialOrd for Entry<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T> Eq for Entry<T> {}
    impl<T> PartialEq for Entry<T> {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }

    let mut queue = BinaryHeap::from_iter(initial.into_iter().map(|item| Entry { item, cost: 0 }));
    let mut visited = HashSet::new();

    while let Some(entry) = queue.pop() {
        if visited.insert(key(&entry.item)) {
            if found(&entry.item) {
                return Some((entry.cost, entry.item));
            }
            for item in edges(entry.item) {
                let cost = entry.cost + 1;
                queue.push(Entry { item, cost });
            }
        }
    }
    None
}