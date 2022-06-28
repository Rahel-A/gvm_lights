import asyncio
import cli_py
import time

def main():
    run()

async def run():
    cli_py.initialise_log()
    test_off = cli_py.PyServerMessage("--light off")
    test_on = cli_py.PyServerMessage("--light on")
    clients = await cli_py.new("10.139.21.199:8631")
    #await clients[0].send_message(test_off)
    t1 = await clients[0].get_state()
    t2 = await clients[1].get_state()
    return [t1, t2]

t = asyncio.run(run())
