# /bin/sh
### BEGIN INIT INFO
# Description: Runs clippy pedantic so I don't have to type the whole command every 4 seconds
### END INIT INFO

# Author: Robin Chapple <r.chapple.business@gmail.com>

cargo clippy -- -W clippy::pedantic
