in:
	docker exec -ti rust /bin/bash

unit_test:
	docker exec -ti rust bash -c "cd $(ARG); cargo test;"

project:
	docker exec -ti rust bash -c "cargo new $(ARG) --lib --vcs none"