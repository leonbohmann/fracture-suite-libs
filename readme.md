# How to build

In `.docker` there is a dockerfile, which can be used to create a docker image:

```bat
cd .docker
podman build -t fracsuite-lib-image -f .\dockerfile
```

Then, save the image:

```bat
podman save -o fracsuite-lib-image.tar lb/fracsuite-lib-image
```

Now there is a .tar file called `fracsuite-lib-image.tar`, which can be uploaded to the MegaNAS server.

Once done, navigate to the destination (currently: `docker/gitea/act_runner`) and load the image:

```bat
sudo docker load -i fracsuite-lib-image.tar
```

In `conner.yaml`, mark the used image as follows:

```yaml
[...]
jobs:
  build_job:
    container:
      image: localhost/fracsuite-lib-image
[...]
```

**Now**, when pushing a new version in `conner/Cargo.toml`, a new release is created!