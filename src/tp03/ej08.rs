// 8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
// puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
// por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
// ella:
// ➔ agregar canción.
// ➔ eliminar canción.
// ➔ mover canción // mueve la canción a una determinada posición de la playlist.
// ➔ buscar canción por nombre.
// ➔ obtener las canciones de un determinado género.
// ➔ obtener las canciones de un determinado artista.
// ➔ modificar título de la playlist.
// ➔ eliminar todas las canciones.x1

#[derive(Debug)]
enum Genero {
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS,
}

impl Genero {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Genero::ROCK, Genero::ROCK)
            | (Genero::POP, Genero::POP)
            | (Genero::RAP, Genero::RAP)
            | (Genero::JAZZ, Genero::JAZZ)
            | (Genero::OTROS, Genero::OTROS) => true,
            _ => false,
        }
    }
    pub fn clone(&self) -> Self {
        match self {
            Genero::ROCK => Genero::ROCK,
            Genero::POP => Genero::POP,
            Genero::RAP => Genero::RAP,
            Genero::JAZZ => Genero::JAZZ,
            Genero::OTROS => Genero::OTROS,
        }
    }
}

#[derive(Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

impl Cancion {
    pub fn equals(&self, other: &Self) -> bool {
        other.titulo == self.titulo
            && self.artista == other.artista
            && self.genero.equals(&other.genero)
    }
    pub fn clone(&self) -> Self {
        Self {
            titulo: self.titulo.clone(),
            artista: self.artista.clone(),
            genero: self.genero.clone(),
        }
    }
}

#[derive(Debug)]
struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Playlist {
    fn get_pos(&mut self, cancion: Cancion) -> usize {
        let mut pos = usize::MAX;
        for i in 0..self.canciones.len() {
            if self.canciones[i].equals(&cancion) {
                pos = i;
                break;
            }
        }
        pos
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, cancion: Cancion) {
        let pos = self.get_pos(cancion);
        if pos != usize::MAX {
            self.canciones.remove(pos);
        }
    }

    pub fn mover_cancion(&mut self, cancion: Cancion, to_index: usize) {
        let posicion_actual = self.get_pos(cancion);
        if to_index < self.canciones.len() {
            if posicion_actual != usize::MAX {
                self.canciones.swap(posicion_actual, to_index);
            }
        } else {
            println!("Posición inválida");
        }
    }

    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        for cancion in &self.canciones {
            if cancion.titulo == nombre {
                return Some(&cancion);
            }
        }
        None
    }

    pub fn obtener_canciones_por_genero<'a>(&self, genero: Genero) -> Vec<&Cancion> {
        let mut return_vec = Vec::new();

        for cancion in &self.canciones {
            if cancion.genero.equals(&genero) {
                return_vec.push(cancion)
            }
        }

        return_vec
    }

    pub fn obtener_canciones_por_artista<'a>(&self, artista: &str) -> Vec<&Cancion> {
        let mut return_vec = Vec::new();

        for cancion in &self.canciones {
            if cancion.artista == artista {
                return_vec.push(cancion)
            }
        }

        return_vec
    }

    pub fn cambiar_nombre(&mut self, new_nombre: String) {
        self.nombre = new_nombre;
    }

    pub fn eliminar_todo(&mut self) {
        self.canciones.clear();
    }
}

pub fn run() {}

mod tests {
    use super::*;

    #[test]
    fn test_agregar_y_eliminar_cancion() {
        let mut playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: Vec::new(),
        };
        let cancion = Cancion {
            titulo: String::from("Cancion 1"),
            artista: String::from("Artista 1"),
            genero: Genero::ROCK,
        };
        let cancion_copia = cancion.clone();

        // Agregar canción
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);

        // Eliminar canción
        playlist.eliminar_cancion(cancion_copia);
        assert_eq!(playlist.canciones.len(), 0);
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: vec![
                Cancion {
                    titulo: String::from("Cancion 1"),
                    artista: String::from("Artista 1"),
                    genero: Genero::ROCK,
                },
                Cancion {
                    titulo: String::from("Cancion 2"),
                    artista: String::from("Artista 2"),
                    genero: Genero::POP,
                },
            ],
        };

        // Mover canción
        let cancion = playlist.canciones[0].clone();
        playlist.mover_cancion(cancion, 1);
        assert_eq!(playlist.canciones[1].titulo, "Cancion 1");
    }

    #[test]
    fn test_buscar_cancion_por_nombre() {
        let playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: vec![
                Cancion {
                    titulo: String::from("Cancion 1"),
                    artista: String::from("Artista 1"),
                    genero: Genero::ROCK,
                },
                Cancion {
                    titulo: String::from("Cancion 2"),
                    artista: String::from("Artista 2"),
                    genero: Genero::POP,
                },
            ],
        };

        let cancion = playlist.buscar_cancion_por_nombre(String::from("Cancion 1"));
        assert!(cancion.is_some());
        assert_eq!(cancion.unwrap().titulo, "Cancion 1");
    }

    #[test]
    fn test_obtener_canciones_por_genero() {
        let playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: vec![
                Cancion {
                    titulo: String::from("Cancion 1"),
                    artista: String::from("Artista 1"),
                    genero: Genero::ROCK,
                },
                Cancion {
                    titulo: String::from("Cancion 2"),
                    artista: String::from("Artista 2"),
                    genero: Genero::POP,
                },
            ],
        };

        let canciones = playlist.obtener_canciones_por_genero(Genero::ROCK);
        assert_eq!(canciones.len(), 1);
        assert_eq!(canciones[0].titulo, "Cancion 1");
    }

    #[test]
    fn test_obtener_canciones_por_artista() {
        let playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: vec![
                Cancion {
                    titulo: String::from("Cancion 1"),
                    artista: String::from("Artista 1"),
                    genero: Genero::ROCK,
                },
                Cancion {
                    titulo: String::from("Cancion 2"),
                    artista: String::from("Artista 2"),
                    genero: Genero::POP,
                },
            ],
        };

        let canciones = playlist.obtener_canciones_por_artista("Artista 1");
        assert_eq!(canciones.len(), 1);
        assert_eq!(canciones[0].titulo, "Cancion 1");
    }

    #[test]
    fn test_cambiar_nombre() {
        let mut playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: Vec::new(),
        };

        playlist.cambiar_nombre(String::from("Nueva Playlist"));
        assert_eq!(playlist.nombre, "Nueva Playlist");
    }

    #[test]
    fn test_eliminar_todo() {
        let mut playlist = Playlist {
            nombre: String::from("Mi Playlist"),
            canciones: vec![
                Cancion {
                    titulo: String::from("Cancion 1"),
                    artista: String::from("Artista 1"),
                    genero: Genero::ROCK,
                },
                Cancion {
                    titulo: String::from("Cancion 2"),
                    artista: String::from("Artista 2"),
                    genero: Genero::POP,
                },
            ],
        };

        playlist.eliminar_todo();
        assert_eq!(playlist.canciones.len(), 0);
    }
}
