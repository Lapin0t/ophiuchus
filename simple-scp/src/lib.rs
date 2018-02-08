use std::ops::BitOr;
use std::hash::Hash;
use std::collections::HashMap;
use std::result;


/// Simple network-less implementation of the SCP state-machine.


pub enum SCPError {}

pub type Result<T> = result::Result<T, SCPError>;


/// Message (transaction) type for the replicated state-machine.
///
/// The SCP machine requires that messages be totally ordered.
pub trait Message: Sized + Ord {}


/// Network driver.
pub trait Driver {
    /// Type of node identifier.
    type Node: Sized + Eq + Hash;

    /// Get a reference to _me_, ie the identity of the local node.
    fn local_node(&self) -> Self::Node;

    /// Test if a given set of nodes forms a quorum.
    ///
    /// A set forms a quorum iff it includes a quorum slice for each of
    /// it's members.
    fn is_quorum(&mut self, nodes: &[Self::Node]) -> bool;

    /// Test if a given set of nodes is blocking for the local node.
    ///
    /// A set is _`v`-blocking_ iff it intersects each quorum slice of `v`.
    fn is_blocking(&mut self, nodes: &[Self::Node]) -> bool;

    /// Get the node with the highest nomination priority.
    ///
    /// This must only be strong pseudo-random function sampling nodes
    /// according to the number of quorum slices of the local node they
    /// appear in.
    fn select_neighbor(&mut self, n: usize, seed: usize) -> Self::Node;
}


/// Ballot container.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Ballot<M: Message> {
    n: usize,
    x: M,
}


impl<M: Message> BitOr for Ballot<M> {
    type Output = bool;

    /// Test compatibility between two ballots.
    ///
    /// Two ballots $a$ and $b$ are compatible ($a \sim b$) iff $a.x = b.x$.
    fn bitor(self, rhs: Self) -> bool {
        self.x == rhs.x
    }
}


/// SCP nomination protocol message.
pub struct MNominate<M: Message> {
    vote: Vec<M>,
    accept: Vec<M>
}


/// SCP ballot protocol message.
pub enum MBallot<M: Message> {
    Prepare { prepare_vote: Ballot<M>,
              prepare_accept_0: Ballot<M>,
              prepare_accept_1: Ballot<M>,
              commit_vote_0n: usize,
              commit_vote_1n: usize },
    Confirm { current: Ballot<M>,
              prepare_accept_n: usize,
              commit_accept_0n: usize,
              commit_accept_1n: usize },
    Externalize { content: M,
                  commit_confirm_0n: usize,
                  commit_confirm_1n: usize }
}


/// State for the SCP machine for a slot.
pub struct State<D: Driver, M: Message> {
    prev: M,
    nomination_msgs: HashMap<D::Node, MNominate<M>>,
    ballot_msgs: HashMap<D::Node, MBallot<M>>
}


impl<D: Driver, M: Message> State<D, M> {
    pub fn new(prev: M) -> Self {
        State { prev: prev,
                nomination_msgs: HashMap::new(),
                ballot_msgs: HashMap::new() }
    }

    pub fn push_nominate(&mut self, src: D::Node, msg: MNominate<M>) -> Result<()> {
        unimplemented!()
    }

    pub fn push_ballot(&mut self, src: D::Node, msg: MBallot<M>) -> Result<()> {
        unimplemented!()
    }
    
    pub fn pull_nominate(&mut self, driver: &mut D) -> Result<MNominate<M>> {
        unimplemented!()
    }

    pub fn pull_ballot(&mut self, driver: &mut D) -> Result<MBallot<M>> {
        unimplemented!()
    }
}
