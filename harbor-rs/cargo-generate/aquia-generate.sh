#!/bin/bash

cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda -n post_member -d tag=member -d import_path=crate::entities -d operation_type=post -d request_type=Member -d response_type=Member
cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda -n get_member -d tag=member -d import_path=crate::entities -d operation_type=get -d request_type=\(\) -d response_type=Member
cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda -n put_member -d tag=member -d import_path=crate::entities -d operation_type=put -d request_type=Member -d response_type=Member
cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda -n delete_member -d tag=member -d import_path=crate::entities -d operation_type=delete -d request_type=\(\) -d response_type=\(\)
cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda -n get_members -d tag=member -d import_path=crate::entities -d operation_type=list -d request_type=\(\) -d response_type=Member
