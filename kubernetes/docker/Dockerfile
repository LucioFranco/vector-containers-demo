FROM luciofranco/vector-kube-demo:latest

RUN apt-get update && apt-get install -y ca-certificates &&  update-ca-certificates

ENTRYPOINT ["vector -c vector.toml"]
