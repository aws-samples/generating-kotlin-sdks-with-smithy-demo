use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use aws_smithy_http_server::Extension;
use leaderboard_server_sdk::error::{CreateLeaderboardError, DeleteLeaderboardError, GetLeaderboardError, UpdateLeaderboardError, NotFoundError, SubmitScoreEventError};
use leaderboard_server_sdk::input::{CreateLeaderboardInput, DeleteLeaderboardInput, GetLeaderboardInput, ListLeaderboardsInput, SubmitScoreEventInput, UpdateLeaderboardInput};
use leaderboard_server_sdk::model::{LeaderboardId, LeaderboardSummary, ScoreEvent};
use leaderboard_server_sdk::output::{CreateLeaderboardOutput, DeleteLeaderboardOutput, GetLeaderboardOutput, ListLeaderboardsOutput, SubmitScoreEventOutput, UpdateLeaderboardOutput};

/// Shared application state
#[derive(Debug, Default)]
pub struct State {
    leaderboards: HashMap<LeaderboardId, LeaderboardSummary>,
    highscores: HashMap<LeaderboardId, Vec<ScoreEvent>>
}

pub async fn submit_score_event_handler(
    input: SubmitScoreEventInput,
    state: Extension<Arc<Mutex<State>>>,
) -> Result<SubmitScoreEventOutput, SubmitScoreEventError> {
    let mut state = state.lock().unwrap();
    let lb = state.leaderboards.get(&input.id);

    match lb {
        Some(lb) => {
            let max_entries = lb.max_entries.to_owned().unwrap().into_inner() as usize;
            let scores = state.highscores.entry(input.id().to_owned()).or_default();

            scores.push(input.score_event().to_owned());
            scores.sort_by_key(|b| std::cmp::Reverse(b.score().unwrap()));
            scores.truncate(max_entries);

            Ok(SubmitScoreEventOutput::builder().build())
        },
        None => Err(
            NotFoundError::builder()
                .message(Some(format!("leaderboard {} not found", input.id().as_str())))
                .build()
                .into()
        )
    }
}

pub async fn create_leaderboard_handler(
    input: CreateLeaderboardInput,
    state: Extension<Arc<Mutex<State>>>,
) -> Result<CreateLeaderboardOutput, CreateLeaderboardError> {
    let mut state = state.lock().unwrap();
    let new_id = format!("leaderboard-{}", state.leaderboards.len());
    // FIXME - there has to be an easier way to go from constraint errors to operation errors
    let lid = LeaderboardId::try_from(new_id).expect("invalid leaderboard id");
    let lb = LeaderboardSummary::builder()
        .id(Some(lid.clone()))
        .name(Some(input.name().to_string()))
        .max_entries(Some(input.max_entries))
        .build();

    state.leaderboards.insert(lid.clone(), lb);

    // FIXME - there has to be an easier way to go from constraint errors to operation errors
    let res = CreateLeaderboardOutput::builder().id(lid).build().unwrap();
    Ok(res)
}

pub async fn get_leaderboard_handler(
    input: GetLeaderboardInput,
    state: Extension<Arc<Mutex<State>>>,
) -> Result<GetLeaderboardOutput, GetLeaderboardError> {
    let state = state.lock().unwrap();
    let lb = state.leaderboards.get(input.id());

    match lb {
        Some(lb) => {
            let name = lb.name.to_owned();
            let scores = state.highscores.get(input.id()).map(|x| x.to_owned());
            let res = GetLeaderboardOutput::builder()
                .name(name)
                .high_scores(scores)
                .build();
            Ok(res)
        },
        None => Err(
            NotFoundError::builder()
                .message(Some(format!("leaderboard {} not found", input.id().as_str())))
                .build()
                .into()
        )
    }
}

pub async fn update_leaderboard_handler(
    _input: UpdateLeaderboardInput,
    _state: Extension<Arc<Mutex<State>>>,
) -> Result<UpdateLeaderboardOutput, UpdateLeaderboardError> {
    todo!("not implemented yet")
}

pub async fn delete_leaderboard_handler(
    input: DeleteLeaderboardInput,
    state: Extension<Arc<Mutex<State>>>,
) -> Result<DeleteLeaderboardOutput, DeleteLeaderboardError> {
    let mut state = state.lock().unwrap();
    if state.leaderboards.contains_key(input.id()) {
        state.leaderboards.remove(input.id());
        Ok(DeleteLeaderboardOutput::builder().build())
    } else {
        Err(
            NotFoundError::builder()
                .message(Some(format!("leaderboard {} not found", input.id().as_str())))
                .build()
                .into()
        )
    }
}

pub async fn list_leaderboards_handler(
    _input: ListLeaderboardsInput,
    state: Extension<Arc<Mutex<State>>>,
) -> ListLeaderboardsOutput {
    let state = state.lock().unwrap();
    // TODO - pagination
    ListLeaderboardsOutput {
        leaderboards: Some(state.leaderboards.values().cloned().collect()),
        next_token: None,
    }
}
