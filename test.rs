#[cfg(test)]
mod tests {
    use crate::{execute, instantiate, query, ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    #[test]
    fn test_counter_contract() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = QueryMsg::GetCounter {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: i32 = from_binary(&res).unwrap();
        assert_eq!(0, value);
        println!("Initial counter value is 0");

        for i in 0..10 {
            let msg = ExecuteMsg::Increment {};
            let info = mock_info("anyone", &[]);
            let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
            assert_eq!(0, res.messages.len());
            let msg = QueryMsg::GetCounter {};
            let res = query(deps.as_ref(), mock_env(), msg).unwrap();
            let value: i32 = from_binary(&res).unwrap();
            assert_eq!(i as i32 + 1, value);
            println!("Counter incremented to {}", value);
        }
    }
}
