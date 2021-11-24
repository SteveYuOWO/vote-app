ID=dev-1637787633523-71365099087082

near view $ID get_round
near view $ID winner
near view $ID get_candidates

near call $ID add_candidate '{"candidate": "steveyu.near"}' --accountId $ID  
near call $ID get_score '{"candidate": "steveyu.near"}' --accountId $ID
near call $ID vote '{"candidate": "steveyu.near"}' --accountId $ID
near call $ID next_round --accountId $ID