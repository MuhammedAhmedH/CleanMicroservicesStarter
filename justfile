up: 
  podman compose up

down:
  podman compose down

clear pod:
  podman rm -f $(podman ps -a) 
