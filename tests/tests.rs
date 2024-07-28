use ranked_voting::ranked_vote;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_wins() {
		let v = vec![
			vec![1,2,3],
			vec![1,2,3],
			vec![1,2,3],
		];
		let result = ranked_vote(v);
		let expectation = (1, false);
        assert_eq!(result, expectation);
    }

	#[test]
	fn untied_elimination_round() {
		let v = vec![
			vec![1,2,3],
			vec![1,2,3],
			vec![1,2,3],
			vec![1,2,3],
			vec![1,2,3],
			vec![2,1,3],
			vec![2,1,3],
			vec![2,1,3],
			vec![2,1,3],
			vec![3,1,2],
			vec![3,1,2],
		];
		let result = ranked_vote(v);
		let expectation = (1, false);
        assert_eq!(result, expectation);
	}

	// this one should randomly remove 2 and 3 without considering 1
	// however if we want to use 1 to determine the prefered eliminator then 3 would be eliminated
	// I've seen two different errors here
	// 1 unwrap fails
	// 2 3 winning (currently a valid solution)
	// TODO TODO TODO this is by far the most complex situation. And due to the nature of write ins it would have a lot.
	#[test]
	fn _tied_elimination_round() {
		let v = vec![
			vec![1],
			vec![1],
			vec![2,3,1],
			vec![3,2,1],
		];
		let result = ranked_vote(v);
		let expectation = (1, true);
        assert_eq!(result, expectation);
	}

	// I would expect a fully random result from this
	// test by running this 100 times and seeing if each value is a potential result
	#[test]
	fn three_way_tie() {
		let v = vec![
			vec![1,2,3],
			vec![2,3,1],
			vec![3,1,2],
		];
		let result = ranked_vote(v);
		let expectation = (1, true);
        assert_eq!(result.1, expectation.1);
	}

	// if two were eliminated I would expect
	/*
		[
			[1,3]
			[1,3]
			[3]
		]
	*/
	#[test]
	fn not_retangular() {
		let v = vec![
			vec![1,2,3],
			vec![1,2,3],
			vec![2],
			vec![3],
		];
		let result = ranked_vote(v);
		let expectation = (1, true);
        assert_eq!(result, expectation);
	}

	#[test]
	#[should_panic]
	fn no_votes() {
		ranked_vote(vec![]);
	}
}
