import asyncio
import gvm_lights_lib
import os

def main():
    run()

async def run():
    gvm_lights_lib.initialise_log()
    test_off = gvm_lights_lib.PyClientArgument("--light off")
    test_on = gvm_lights_lib.PyClientArgument("--light on")
    test_br1 = gvm_lights_lib.PyClientArgument("--brightness 50")
    test_br2 = gvm_lights_lib.PyClientArgument("--brightness 5")
    clients = await gvm_lights_lib.new(f"{APP_HOST}:{APP_PORT}")
    await clients[0].send_message(test_on)
    #await clients[1].send_message(test_on)
    #await clients[0].send_message(test_on)
    #await clients[1].send_message(test_on)
    #await clients[0].send_message(test_br1)
    #await clients[0].send_message(test_br2)
    t1 = await clients[0].get_state()
    #t2 = await clients[1].get_state()
    #return [t1, t2]
    return [t1]

APP_HOST = os.environ.get('APP_HOST', '0.0.0.0')
APP_PORT = os.environ.get('APP_PORT', '8631')

t = asyncio.run(run())
