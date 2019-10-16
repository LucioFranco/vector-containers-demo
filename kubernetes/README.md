# Install vector

Create vector config, this must go first to allow the vector dameonset pick it up.

```shell
kubectl apply -f kube/vector-config.yaml
```

We must also create the secrets via `kubectl`

```shell
kubectl --namespace telemetry \
    create secret generic vector-cloudwatch \
    --from-file=./secrets/vector-cloudwatch-iam
```

Now we can deploy the vector agent daemon set

```shell
kubectl apply -f kube/vector-agent-daemonset.yaml
```

# Install Echo Service

```shell
kubectl apply -f kube/echo.yaml
```

Now lets find the port exposed by the service

```shell
kubectl get services --output json | jq '.items[0].spec.ports[0].nodePort'
```

Now curl it so it can generate logs!

```shell
curl http://$(minikube ip):<service-port>/
```
