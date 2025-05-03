use crate::{Rng, fuzz::RandomInstance};

const MAX_ELEMENTS: usize = 512;

impl<T: RandomInstance> RandomInstance for Option<T> {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.5) {
            return None;
        }

        Some(T::random_instance(rng))
    }
}

impl<T: RandomInstance> RandomInstance for Vec<T> {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        let len = rng.gen_range(0..MAX_ELEMENTS);
        let mut vec = Vec::new();

        for _ in 0..len {
            vec.push(T::random_instance(rng));
        }

        vec
    }
}

impl<T: RandomInstance> RandomInstance for std::collections::VecDeque<T> {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        let len = rng.gen_range(0..MAX_ELEMENTS);
        let mut vec = std::collections::VecDeque::new();

        for _ in 0..len {
            vec.push_back(T::random_instance(rng));
        }

        vec
    }
}

// impl<T> RandomInstance for Range<T> {
//     #[inline]
//     fn auto_map_entities<M: EntityMapper>(&mut self, _: &mut M) {
//         // Range<T> is considered primitive-like, so do nothing
//     }
// }

// impl<T: RandomInstance + Hash + Eq> RandomInstance for HashSet<T> {
//     fn random_instance(rng: &mut Rng) -> Self {
//         let len = rng.gen_range(0..COLLECTION_LEN);
//         let mut set = HashSet::new();

//         for _ in 0..len {
//             set.insert(T::random_instance(rng));
//         }

//         set
//     }
// }

// impl<K: RandomInstance + Hash + Eq, V: RandomInstance> RandomInstance for HashMap<K, V> {
//     fn random_instance(rng: &mut Rng) -> Self {
//         let len = rng.gen_range(0..COLLECTION_LEN);
//         let mut map = HashMap::new();

//         for _ in 0..len {
//             map.insert(K::random_instance(rng), V::random_instance(rng));
//         }

//         map
//     }
// }

impl<T1: RandomInstance, T2: RandomInstance> RandomInstance for (T1, T2) {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        (T1::random_instance(rng), T2::random_instance(rng))
    }
}

impl<T1: RandomInstance, T2: RandomInstance, T3: RandomInstance> RandomInstance for (T1, T2, T3) {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        (
            T1::random_instance(rng),
            T2::random_instance(rng),
            T3::random_instance(rng),
        )
    }
}

impl<T1: RandomInstance, T2: RandomInstance, T3: RandomInstance, T4: RandomInstance> RandomInstance
    for (T1, T2, T3, T4)
{
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        (
            T1::random_instance(rng),
            T2::random_instance(rng),
            T3::random_instance(rng),
            T4::random_instance(rng),
        )
    }
}

impl<T: RandomInstance + Clone, const N: usize> RandomInstance for [T; N] {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        let arr = [0; N];

        arr.map(|_| T::random_instance(rng))
    }
}
