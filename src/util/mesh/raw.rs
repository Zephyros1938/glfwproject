use std::collections::HashMap;

pub struct MeshRaw<T: 'static + std::cmp::Eq + std::hash::Hash + Clone> {
    data: &'static [T],
}

impl<T: 'static + std::cmp::Eq + std::hash::Hash + Clone> MeshRaw<T> {
    pub fn new(data: &'static [T]) -> Self {
        Self { data }
    }
    pub fn get_indiced_data(&self) -> (Vec<T>, Vec<u32>) {
        let mut unique_vertices: Vec<T> = Vec::new();
        let mut vertex_to_index: HashMap<T, u32> = HashMap::new();
        let mut indices: Vec<u32> = Vec::new();

        for vertex in self.data {
            if let Some(&index) = vertex_to_index.get(vertex) {
                indices.push(index);
            } else {
                let index = unique_vertices.len() as u32;
                unique_vertices.push(vertex.clone());
                vertex_to_index.insert(vertex.clone(), index);
                indices.push(index);
            }
        }

        (unique_vertices, indices)
    }
}

#[derive(Debug)]
pub struct Mesh<T> {
    unique_vertices: Vec<T>,
    indices: Vec<u32>,
}

impl<T: 'static + Eq + std::hash::Hash + Clone> Mesh<T> {
    /// Constructs a new Mesh from the given array.
    /// It collects unique vertices and builds an index map.
    pub fn new(data: &[T]) -> Self {
        let mut unique_vertices = Vec::new();
        let mut indices = Vec::with_capacity(data.len());

        for item in data.iter() {
            let pos = if let Some(idx) = unique_vertices.iter().position(|v| v == item) {
                idx as u32
            } else {
                unique_vertices.push(item.clone());
                (unique_vertices.len() - 1) as u32
            };
            indices.push(pos);
        }

        Self {
            unique_vertices,
            indices,
        }
    }

    /// Returns a array of the unique vertices.
    pub fn get_data(&self) -> &[T] {
        self.unique_vertices.as_slice()
    }

    /// Returns a array of the indices.
    pub fn get_indices(&self) -> &[u32] {
        self.indices.as_slice()
    }

    /// Updates the index at `pos` with the given data.
    ///
    /// If the data is already present in `unique_vertices`,
    /// its position is used. Else, the data is appended to `unique_vertices` and its new index is used.
    pub fn set_data(&mut self, pos: usize, data: T) {
        // Try to find the position of data in the unique vertices.
        let index = if let Some(idx) = self.unique_vertices.iter().position(|v| *v == data) {
            idx
        } else {
            // not present: insert use and new index.
            self.unique_vertices.push(data);
            self.unique_vertices.len() - 1
        };
        // update the index at the given position.
        self.indices[pos] = index as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duplicates() {
        let start = std::time::Instant::now();
        let data = &[10, 20, 30, 40];
        let mesh = MeshRaw::new(data);
        let (unique, indices) = mesh.get_indiced_data();

        assert_eq!(unique, vec![10, 20, 30, 40]);
        assert_eq!(indices, vec![0, 1, 2, 3]);
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_all_duplicates() {
        let start = std::time::Instant::now();
        let data = &[7, 7, 7, 7];
        let mesh = Mesh::new(data);
        use std::collections::HashMap;

        pub struct Mesh<T: 'static + std::cmp::Eq + std::hash::Hash + Clone> {
            data: &'static [T],
        }

        impl<T: 'static + std::cmp::Eq + std::hash::Hash + Clone> Mesh<T> {
            pub fn new(data: &'static [T]) -> Self {
                Self { data }
            }
            pub fn get_indiced_data(&self) -> (Vec<T>, Vec<u32>) {
                let mut unique_vertices: Vec<T> = Vec::new();
                let mut vertex_to_index: HashMap<T, u32> = HashMap::new();
                let mut indices: Vec<u32> = Vec::new();

                for vertex in self.data {
                    if let Some(&index) = vertex_to_index.get(vertex) {
                        indices.push(index);
                    } else {
                        let index = unique_vertices.len() as u32;
                        unique_vertices.push(vertex.clone());
                        vertex_to_index.insert(vertex.clone(), index);
                        indices.push(index);
                    }
                }

                (unique_vertices, indices)
            }
        }
        let (unique, indices) = mesh.get_indiced_data();

        assert_eq!(unique, vec![7]);
        assert_eq!(indices, vec![0, 0, 0, 0]);
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_mixed_duplicates() {
        let start = std::time::Instant::now();
        let data = &[5, 2, 5, 3, 2, 3, 2, 5];
        let mesh = MeshRaw::new(data);
        let (unique, indices) = mesh.get_indiced_data();

        // Order of first appearance: 5, 2, 3
        assert_eq!(unique, vec![5, 2, 3]);
        assert_eq!(indices, vec![0, 1, 0, 2, 1, 2, 1, 0]);
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_single_element() {
        let start = std::time::Instant::now();
        let data = &[99];
        let mesh = MeshRaw::new(data);
        let (unique, indices) = mesh.get_indiced_data();

        assert_eq!(unique, vec![99]);
        assert_eq!(indices, vec![0]);
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_empty_input() {
        let start = std::time::Instant::now();
        let data: &[u32] = &[];
        let mesh = MeshRaw::new(data);
        let (unique, indices) = mesh.get_indiced_data();

        assert_eq!(unique, Vec::<u32>::new());
        assert_eq!(indices, Vec::<u32>::new());
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_large_input() {
        let start = std::time::Instant::now();
        // Generate 1 million values cycling through 0..1000
        let large_data: Vec<u32> = (0..1_000_000).map(|i| i % 1000).collect();
        let mesh = MeshRaw::new(Box::leak(large_data.into_boxed_slice())); // convert to &'static [u32]
        let (unique, indices) = mesh.get_indiced_data();

        // Ensure unique values are 0..999
        let expected_unique: Vec<u32> = (0..1000).collect();
        assert_eq!(unique, expected_unique);

        // Ensure indices correctly wrap around every 1000
        assert_eq!(indices.len(), 1_000_000);
        for (i, index) in indices.iter().enumerate().take(10_000) {
            assert_eq!(*index, (i % 1000) as u32);
        }
        println!("Took {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_large_inputs() {
        for &n in &[
            // notice the ampersand to iterate over references
            1_000_000_u32,
            2_500_000_u32,
            5_000_000_u32,
            10_000_000_u32,
            20_500_000_u32,
            50_000_000_u32,
            100_000_000_u32,
            200_500_000_u32,
            500_000_000_u32,
            1_000_000_000_u32,
        ] {
            let start = std::time::Instant::now();
            let large_data: Vec<u32> = (0..n).map(|i| i % 1000).collect();
            let mesh = MeshRaw::new(Box::leak(large_data.into_boxed_slice()));
            let (unique, indices) = mesh.get_indiced_data();

            // Only check unique list once since the same 0..1000 pattern repeats
            let expected_unique: Vec<u32> = (0..1000).collect();
            assert_eq!(unique, expected_unique);

            // Check first 10,000 indices
            assert_eq!(indices.len(), n as usize);
            for (i, index) in indices.iter().enumerate().take(10_000) {
                assert_eq!(*index, (i % 1000) as u32);
            }
            println!("Took {}ms for {}", start.elapsed().as_millis(), n);
        }
    }
    #[test]
    fn test_mesh_datasetter() {
        let mut s = Mesh::new(&[
            'a', 'b', 'c', 'a', 'b', 'c', 'd', 'e', 'f', 'a', 'e', 'f', 'g', 'h', 'i', 'g', 'h',
            'i', 'a', 'b', 'c', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        ]);

        println!("Initial unique vertices: {:?}", s.get_data());
        println!("Initial indices: {:?}", s.get_indices());

        // Apply a lot of set_data operations
        s.set_data(0, 'z'); // replaces 'a'
        s.set_data(1, 'y'); // replaces 'b'
        s.set_data(2, 'x'); // replaces 'c'
        s.set_data(3, 'z'); // should reuse index of 'z'
        s.set_data(4, 'y'); // should reuse index of 'y'
        s.set_data(5, 'x'); // should reuse index of 'x'

        s.set_data(10, 'w'); // replaces 'e'
        s.set_data(11, 'v'); // replaces 'f'
        s.set_data(12, 'u'); // replaces 'g'
        s.set_data(13, 't'); // replaces 'h'
        s.set_data(14, 's'); // replaces 'i'

        s.set_data(15, 'z'); // reuse again
        s.set_data(16, 'y'); // reuse again
        s.set_data(17, 'x'); // reuse again

        println!("\nAfter many set_data calls:");
        println!("Updated unique vertices: {:?}", s.get_data());
        println!("Updated indices: {:?}", s.get_indices());

        // Check that the index values match the position in `unique_vertices`
        let data = s.get_data();
        let indices = s.get_indices();

        for (i, &index) in indices.iter().enumerate() {
            assert!(
                index < data.len() as u32,
                "Index {} out of bounds at position {}",
                index,
                i
            );
        }

        // Just verify some expected remapping
        let unique = s.get_data();
        let indices = s.get_indices();

        assert_eq!(unique[s.get_indices()[0] as usize], 'z');
        assert_eq!(unique[s.get_indices()[1] as usize], 'y');
        assert_eq!(unique[s.get_indices()[2] as usize], 'x');
        assert_eq!(unique[s.get_indices()[10] as usize], 'w');
        assert_eq!(unique[s.get_indices()[14] as usize], 's');

        println!("\nAll assertions passed!");
    }
}
