import sys 
import anyio
import dagger
import os
from decouple import config


async def pipeline():
    async with dagger.Connection(dagger.Config(log_output=sys.stderr)) as client:
        src = client.host().directory(".")
        # build svelts & copy build dir to /src/web
        print("running 'npm run build' on web ui")
        web = (
            client.container().from_("node:18-alpine3.16")
                .with_mounted_directory("/app", src)
                .with_workdir("/app/src/ui")
                .with_exec(["npm" ,"i"])
                .with_exec(["npm", "run", "build"])
        )
        svelt_output = await web.stdout()
        static_files = web.directory("./build")
        
        # build rust server w/ static web files
        rust = (
            client.container().from_("rust:1.68-buster")
                .with_mounted_directory("/app", src)
                .with_workdir("/app")
                .with_directory("/app/src/web", static_files)
                .with_exec(["cargo", "build", "--release"])
        )
        rust_output = await rust.directory("./target/release").export("./build")
        app = (
            client.container()
                .with_mounted_directory("/app", src)
                .with_workdir("/app")
                .build(src)
                .publish("docker.io/anthonymolinari/ws-and-api:latest")
        )
        await app

    print('done.')


def dockerLogin():
     USERNAME = os.getenv('DOCKERHUB_USERNAME')
     TOKEN = os.getenv('DOCKERHUB_TOKEN')

     if USERNAME is None or TOKEN is None:
          # use local .env
          USERNAME = config('DOCKERHUB_USERNAME')
          TOKEN = config('DOCKERHUB_TOKEN')

     if os.system(f'docker login -u {USERNAME} -p {TOKEN}') != 0:
          print(f'Error login failed')
          sys.exit(1)
     

def dockerLogout():
     if os.system('docker logout') != 0:
          print(f'Error logout failed')
          sys.exit(1)
     

if __name__ == "__main__":
    dockerLogin()
    # wrap in try catch block
    anyio.run(pipeline)
    dockerLogout()