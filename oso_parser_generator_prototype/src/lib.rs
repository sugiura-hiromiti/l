#![no_std]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]

pub struct Tree<'a, N: NodeValue,> {
	value:    N,
	children: &'a [Self],
	parent:   Option<&'a Self,>,
}

#[const_trait]
pub trait TreeWindow: TreeWalk {
	type Children: TreeWalk;
	type Brothers: TreeWalk;
	fn children<WT: WalkTried<T = Self::Children,>,>(&mut self,) -> WT;
	fn brothers<WT: WalkTried<T = Self::Brothers,>,>(&mut self,) -> WT;
}

pub trait TreeWalk: Sized + Iterator {
	type N: NodeValue;
	type O: TreeWalk;

	fn current(&self,) -> impl TreeWalk;

	fn parent<WT: WalkTried<T = Self::O,>,>(&mut self,) -> WT;
	fn nth_ancestor<WT: WalkTried<T = Self::O,>,>(&mut self, n: usize,) -> WT {
		if n == 0 {
			self.as_walk_tried()
		} else {
			let mut parent = self.parent::<WT>();
			if parent.has_success() {
				parent.current_tree_mut().as_mut().unwrap().nth_ancestor::<WT>(n - 1,)
			} else {
				parent
			}
		}
	}

	fn set_pos<WT: WalkTried,>(&mut self, coordinate: impl Coordinate,) -> WT;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried<WT: WalkTried,>(&self,) -> WT;

	fn value(&self,) -> <<Self as TreeWalk>::N as NodeValue>::Output;
	fn node(&self,) -> Self::N;
}

pub trait WalkTried {
	type T: TreeWalk;
	type C: Coordinate;
	// type TreeNode: TreeWalk<'a, Self::N,>
	// where Self::N: 'a;

	fn has_success(&self,) -> bool;
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	fn last_valid_coordinate(&self,) -> &Self::C;
	fn current_tree(&self,) -> &Option<Self::T,>;
	fn current_tree_mut(&mut self,) -> &mut Option<Self::T,>;

	fn from(tn: Self::T, coord: Self::C,) -> Self;
}

pub trait Coordinate {
	fn nth_dimension(&self, n: usize,) -> usize;
	fn first_dimension(&self,) -> usize {
		self.nth_dimension(0,)
	}
	fn last_dimension(&self,) -> usize {
		let last_dimension_is = self.dimension_count();
		self.nth_dimension(last_dimension_is - 1,)
	}

	fn dimension_count(&self,) -> usize;
	fn set_at(&mut self, dim: usize, value: usize,);
}

pub struct Node<T: Clone,>(T,);

pub trait NodeValue: AsMut<Self::Output,> + AsRef<Self::Output,>
where Self::Output: Clone
{
	type Output;
	fn clone_value(&self,) -> Self::Output;
}

impl<T: Clone,> NodeValue for Node<T,> {
	type Output = T;

	fn clone_value(&self,) -> Self::Output {
		self.0.clone()
	}
}

impl<T: Clone,> AsRef<T,> for Node<T,> {
	fn as_ref(&self,) -> &T {
		&self.0
	}
}

impl<T: Clone,> AsMut<T,> for Node<T,> {
	fn as_mut(&mut self,) -> &mut T {
		&mut self.0
	}
}

pub struct WalkRslt<T: TreeWalk, C: Coordinate,> {
	// __constraint: core::marker::PhantomData<N,>,
	tree:  Option<T,>,
	coord: C,
}

impl<T: TreeWalk, C: Coordinate,> WalkTried for WalkRslt<T, C,> {
	type C = C;
	type T = T;

	fn has_success(&self,) -> bool {
		self.tree.is_some()
	}

	fn last_valid_coordinate(&self,) -> &Self::C {
		&self.coord
	}

	fn current_tree(&self,) -> &Option<Self::T,> {
		&self.tree
	}

	fn current_tree_mut(&mut self,) -> &mut Option<Self::T,> {
		&mut self.tree
	}

	fn from(tn: T, coord: Self::C,) -> Self {
		Self {
			//__constraint: core::marker::PhantomData::<N,>,
			tree: Some(tn,),
			coord,
		}
	}
}
