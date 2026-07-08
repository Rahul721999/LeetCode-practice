use crate::lru_cache::LRUCache;

mod doubly_linked_list;
mod lru_cache;

fn main() {
    let mut lru_cache: LRUCache<&str, i32> = LRUCache::new(2);

        lru_cache.put("person1", 1);
        lru_cache.put("person2", 2);

        assert_eq!(lru_cache.get("person1"), Some(1));

        lru_cache.put("person3", 3);
        
        // person2 should be evicted because person1 was just accessed
        assert_eq!(lru_cache.get("person2"), None);
        
        lru_cache.remove("person1");
        assert_eq!(lru_cache.get("person1"), None);
        
        assert_eq!(lru_cache.get("person3"), Some(3));
}