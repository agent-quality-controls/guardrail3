// Adversarial fixture: R40 boundary test
// Exactly 20 use statements. R40 fires on > 20, NOT >= 20.
// This file should NOT trigger R40.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::BinaryHeap;
use std::fmt;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::str::FromStr;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ops::Deref;
use std::ops::DerefMut;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::time::Duration;
use std::time::Instant;

fn main() {}
