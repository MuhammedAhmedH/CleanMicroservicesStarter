up: 
  podman compose up -d

down:
  podman compose down

clear pod:
  podman rm -f $(podman ps -a) 
