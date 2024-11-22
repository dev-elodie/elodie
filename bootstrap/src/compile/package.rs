use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use crate::{ast, parse};
use crate::ast::{DeclarePackageNode, Identifier};
use crate::compile::{compile_str, Compiler};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_package(&mut self, node: &parse::PackageDeclarationNode) -> crate::compile::Result<ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.compile_node(node)?);
        }

        let mut packages = vec![];
        for node in &compiled_body {
            if let ast::Node::Block(block) = node {
                for node in &block.body {
                    if let ast::Node::ExportPackage(_) = node {
                        packages.append(self.load_declared_packages("FIXME").as_mut());
                    }
                }
            }
        }

        Ok(ast::Node::DeclarePackage(DeclarePackageNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions: compiled_body.into_iter()
                .filter_map(|n| {
                    if let ast::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function) // Now directly taking ownership
                    } else {
                        None
                    }
                })
                .collect(),
            packages,
        }))
    }


    fn load_declared_packages(&mut self, name: &str) -> Vec<DeclarePackageNode> {
        let content = crate::load_library_file("std/io/index.elx").unwrap();
        let src_file = compile_str(&mut self.ctx, content.as_str()).unwrap();

        let mut result = vec![];

        for node in src_file.body {
            if let ast::Node::DeclarePackage(package_node) = node {
                result.push(package_node);
            }
        }

        result
    }
}

fn load_library_file(filename: &str) -> io::Result<String> {
    // Get the path to the project root directory
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";

    // Construct the full path to the file
    let file_path = PathBuf::from(manifest_dir).join(filename);
    // println!("{file_path:?}");

    // Read the file's contents
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{contents}");
    Ok(contents)
}