//! The ordinary herdr-nexus contract reads and prints datom text, and refuses wrong shapes.

#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Composing, Datomizable, Path, Potential};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal_herdr_nexus::{Query, Response};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).expect("the reader budget is within range"),
        depth: 0,
        maximum_depth: 4096,
    }
}

fn read<T: Composing>(source: &str) -> Result<T, datom_codec::Error> {
    Potential::<T>::from(source).actualize(&mut budget())
}

fn print<T: Datomizable>(value: &T) -> String {
    value.datomize(Path::new()).protosize().compact()
}

fn round_trip<T: Composing + Datomizable + std::fmt::Debug + PartialEq>(source: &str) {
    let value = read::<T>(source).unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(
        read::<T>(&print(&value)).expect("the printing reads back"),
        value,
        "{source}"
    );
}

#[test]
fn every_query_shape_round_trips() {
    round_trip::<Query>("Close.{ main wQ:pT }");
    round_trip::<Query>("List.main");
    round_trip::<Query>("Subscribe.main");
    round_trip::<Query>("Press.{ { main wQ:pT } [ Enter ] }");
    round_trip::<Query>("Prompt.{ { main wQ:pT } «hello world» }");
}

#[test]
fn a_response_round_trips() {
    round_trip::<Response>("Changed.{ { main wQ:pT } Idle }");
}

#[test]
fn wrong_shapes_and_unknown_variants_are_refused() {
    assert!(read::<Query>("Close.main").is_err());
    assert!(read::<Query>("Frobnicate.main").is_err());
    assert!(read::<Query>("Press.{ main }").is_err());
    assert!(read::<Response>("Changed.{ { main wQ:pT } Sleeping }").is_err());
}
