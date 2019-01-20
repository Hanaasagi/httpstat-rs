build:
	echo "Building httpstat ..."
	mkdir -p ./dist
	docker build . -t httpstat:build
	docker create --name httpstat-extract httpstat:build
	docker cp httpstat-extract:/usr/src/dist/release/httpstat-rs ./dist/
	docker rm -f httpstat-extract
	echo "Finished."

install: build
	echo "Installing ..."
	cp ./dist/httpstat-rs /usr/local/bin/httpstat
	echo "Finished."

clean:
	echo "Cleaning ..."
	rm -rf ./dist
	docker rmi httpstat:build
	echo "Finished."

uninstall:
	echo "Uninstalling ..."
	rm -f /usr/local/bin/httpstat
	echo "Finished."

.SLIENT: build install clean uninstall
.PHONY: build install clean uninstall
