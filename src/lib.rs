use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::collections::HashMap;

// A record is the ranked order of the selections for a given voter
// This is the example for no weighted votes
// note this will only work if everyone votes with all the choices
pub fn ranked_vote(mut vote_matrix: Vec<Vec<usize>>) -> (usize, bool) {
	let mut randomness = false;

	loop {
		let mut tally = HashMap::new();

		for voter in &vote_matrix {
			if voter.len() > 0 {
				let highest_priority_vote = voter[0];
				// Add a vote to the Entry
				// When the Entry is empty set the value to 1
				tally.entry(highest_priority_vote).and_modify(|count| *count += 1).or_insert(1);
			}
		}

		match majority_winner(&tally) {
			Some(e) => return (e, randomness),
			None => {
				let retval = elimination_round(&mut vote_matrix, &mut tally);
				if retval { randomness = true; }
			},
		}
	}
}

// returns the majority winner if there is a majority
fn majority_winner(tallied_votes: &HashMap<usize, usize>) -> Option<usize> {
	let mut total_votes = 0;

	for (_, votes) in tallied_votes {
		total_votes += votes;
	}

	for (canidate, votes) in tallied_votes {
		if total_votes / votes == 1 { // 2 / 1 == 2 which is not a majority 3 / 2 == 1 meaning we have a majority
			return Some(canidate.clone()); // I feel like I can live with a clone since it will be the return value and the matrix is not owned anymore
		}
	}

	None
}

// TODO this is going to need a lot of thought and testing
fn elimination_round(vote_matrix: &mut Vec<Vec<usize>>, tally: &HashMap<usize, usize>) -> bool {
	let mut was_randomness_used = false;
	let mut tally_iter: Vec<(usize, usize)> = tally.clone().into_iter().collect(); //warning! // why can't the type be inferred here?
	let first_element = tally_iter.pop().unwrap(); // warning!
	let mut canidate_with_least_votes = first_element.0;
	let mut min_votes = first_element.1;

	// TODO improve random
	// This is not a good way to do random but there is no standard random library at this time
	// Weak randomness due to user control of when the program is run
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

	for (canidate, votes) in tally_iter {
		if min_votes == votes {
			if since_the_epoch.as_millis() % 2 == 0 {
				canidate_with_least_votes = canidate;
				min_votes = votes;
			}
			was_randomness_used = true;
		} else if min_votes > votes {
			canidate_with_least_votes = canidate;
			min_votes = votes;
		}
	}

	eliminate(&canidate_with_least_votes, vote_matrix);

	was_randomness_used
}

fn eliminate(canidate: &usize, vote_matrix: &mut Vec<Vec<usize>>) {
	for canidate_votes in vote_matrix {
		let index = canidate_votes.iter().position(|&element| element == *canidate);
		match index {
			Some(i) => { canidate_votes.remove(i); },
			None => (),
		}
	}
}