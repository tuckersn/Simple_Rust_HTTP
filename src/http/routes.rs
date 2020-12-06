use std::{collections::{HashMap, HashSet}, marker::Send, pin::Pin};
use std::io::{Error, ErrorKind};
use std::result::Result;
use std::fmt;
use std::clone::Clone;
use futures::Future;

use super::{request::Request, response::Response};




pub type RouteFn<T, F> = Box<
	dyn Fn(T, F) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + Sync>> + Send + Sync
>;


pub struct RouteNode {
	pub name: String,
	pub children: HashMap<String, RouteNode>,
	pub function: Option<RouteFn<Request, Response>>
}

impl RouteNode {

	pub fn new(name: &str, function: Option<RouteFn<Request, Response>>) -> RouteNode {
		RouteNode {
			name: name.to_string(),
			children: HashMap::new(),
			function
		}
	}

	fn add(&mut self, name: &str, function: Option<RouteFn<Request, Response>>) -> &RouteNode {
		let node = RouteNode::new(name, function);
		self.children.insert(name.to_string(), node);
		self.children.get(name).unwrap()
	}

	pub fn function(&self) -> Option<&RouteFn<Request, Response>> {
		match &self.function {
			Some(fun) => {
				Some(fun)
			}
			None => {
				None
			}
		}
	}
}


impl fmt::Debug for RouteNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RouteNode {{
			name: {}
		}}", &self.name)
	}
}


async fn testFn(req: Request, res: Response) {

}

pub struct Router {
	root: RouteNode
}

impl Router {

	pub fn new() -> Router {
		Router {
			root: RouteNode::new("", None)
		}
	}


	pub fn register<RoutingFunction, RoutingFuture>(&mut self, path: impl Into<String>, function: RoutingFunction) -> &Router
	where
		RoutingFunction: (Fn(Request, Response) -> RoutingFuture) + Send + Sync + 'static,
		RoutingFuture: Future<Output = Result<(), Error>> + Send + Sync + 'static,
	{
		let path = path.into();
		let segments: Vec<&str> = path.split('/').skip(1).collect();

		let mut node = &mut self.root;
		for seg in segments {

			if seg.split_at(1) == b'{' {

			}

			if !node.children.contains_key(seg) {
				node.children.insert(seg.to_string(), RouteNode::new(seg, None));
			}
			node = node.children.get_mut(seg).unwrap();
		}
		node.function = Some(Box::new(move |req: Request, res: Response| {
			Box::pin(function(req,res))
		}));

		self
	}

	pub fn find(&self, path: impl Into<String>) -> Result<&RouteNode, Error> {

		let path = path.into();
		let segments: Vec<&str> = path.split('/').skip(1).collect();

		let mut node = &self.root;
		for seg in segments {
			if node.children.contains_key(seg) {
				node = node.children.get(seg).unwrap();
			} else {
				return Err(Error::new(ErrorKind::NotFound, "Invalid URL"))
			}
		}

		Ok(node)
	}
}
