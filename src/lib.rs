#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod biblioteca_storage {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use core::convert::TryFrom;

    /// Definição dos gêneros dos livros.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Genero {
        Ficcao = 0,
        Biografia = 1,
        Poesia = 2,
        Infantil = 3,
        Romance = 4,
        Outro = 5,
    }

    impl TryFrom<u8> for Genero {
        type Error = &'static str;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Genero::Ficcao),
                1 => Ok(Genero::Biografia),
                2 => Ok(Genero::Poesia),
                3 => Ok(Genero::Infantil),
                4 => Ok(Genero::Romance),
                5 => Ok(Genero::Outro),
                _ => Err("Valor inválido para Genero"),
            }
        }
    }

    /// Estrutura de um livro.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Livro {
        id: u32,
        titulo: String,
        genero: Genero,
    }

    #[ink(storage)]
    pub struct BibliotecaStorage {
        livros: Vec<Livro>,
        proximo_id: u32,
    }

    impl BibliotecaStorage {
        /// Construtor do contrato.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                livros: Vec::new(),
                proximo_id: 1,
            }
        }

        /// Adiciona um novo livro à biblioteca.
        #[ink(message)]
        pub fn adicionar_livro(&mut self, titulo: String, genero: Genero) -> u32 {
            let livro = Livro {
                id: self.proximo_id,
                titulo,
                genero,
            };
            self.livros.push(livro);
            let id_atual = self.proximo_id;
            self.proximo_id = self.proximo_id.saturating_add(1);
            id_atual // Retorna o ID do livro adicionado
        }

        /// Retorna a lista de livros cadastrados.
        #[ink(message)]
        pub fn listar_livros(&self) -> Vec<Livro> {
            self.livros.clone()
        }

        /// Atualiza um livro existente pelo ID.
        #[ink(message)]
        pub fn atualizar_livro(&mut self, id: u32, novo_titulo: String, novo_genero: Genero) -> bool {
            for livro in &mut self.livros {
                if livro.id == id {
                    livro.titulo = novo_titulo;
                    livro.genero = novo_genero;
                    return true;
                }
            }
            false
        }

        /// Remove um livro pelo ID.
        #[ink(message)]
        pub fn remover_livro(&mut self, id: u32) -> bool {
            if let Some(index) = self.livros.iter().position(|livro| livro.id == id) {
                self.livros.remove(index);
                return true;
            }
            false
        }
    }

    /// Testes para verificar o funcionamento do contrato
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_adicionar_livro() {
            let mut contract = BibliotecaStorage::new();
            let id = contract.adicionar_livro("Livro A".into(), Genero::Ficcao);
            assert_eq!(id, 1);
        }

        #[ink::test]
        fn test_listar_livros() {
            let mut contract = BibliotecaStorage::new();
            contract.adicionar_livro("Livro A".into(), Genero::Ficcao);
            let livros = contract.listar_livros();
            assert_eq!(livros.len(), 1);
            assert_eq!(livros[0].titulo, "Livro A");
        }

        #[ink::test]
        fn test_atualizar_livro() {
            let mut contract = BibliotecaStorage::new();
            let id = contract.adicionar_livro("Antigo".into(), Genero::Ficcao);
            let atualizado = contract.atualizar_livro(id, "Novo".into(), Genero::Romance);
            assert!(atualizado);
            let livros = contract.listar_livros();
            assert_eq!(livros[0].titulo, "Novo");
        }

        #[ink::test]
        fn test_remover_livro() {
            let mut contract = BibliotecaStorage::new();
            let id = contract.adicionar_livro("Livro Removível".into(), Genero::Outro);
            assert_eq!(contract.listar_livros().len(), 1);
            let removido = contract.remover_livro(id);
            assert!(removido);
            assert_eq!(contract.listar_livros().len(), 0);
        }
    }
}
