# Lab 4D – Kubernetes WASM Manifests

These YAML files deploy a WASM workload on a Kubernetes cluster configured with the **runwasi** containerd shim.

## Files

| File | Purpose |
|------|---------|
| `runtime-class.yaml` | Registers `wasmtime` as a K8s RuntimeClass |
| `deployment.yaml` | Deploys 3 replicas of a WASM microservice |
| `service.yaml` | Exposes the service on port 80 (ClusterIP) |
| `kustomization.yaml` | Kustomize bundle for all resources |

## Apply to a Cluster (if available)

```bash
# Apply with kubectl
kubectl apply -f runtime-class.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml

# Or with kustomize
kubectl apply -k .

# Check rollout
kubectl rollout status deployment/wasm-microservice

# View pods (should be Running almost instantly)
kubectl get pods -l app=wasm-microservice

# Test via port-forward
kubectl port-forward svc/wasm-microservice 8080:80
curl http://localhost:8080/health
```

## Cluster Setup Requirements

To run WASM workloads on Kubernetes you need:

1. **containerd** (≥ 1.7) as the container runtime
2. **runwasi** shim installed on each node:
   ```bash
   # On each K8s node
   curl -sSL https://github.com/containerd/runwasi/releases/latest/download/containerd-shim-wasmtime-v1-linux-amd64.tar.gz | tar xz -C /usr/local/bin
   ```
3. Nodes labelled: `kubectl label node <node-name> runtime=wasm`
4. containerd configured to recognize the shim

## Discussion Points

- Why is `initialDelaySeconds: 1` realistic for WASM but unusual for containers?
- What does `memory: "32Mi"` tell us about WASM's resource footprint?
- How would you build and push the OCI image containing a `.wasm` binary?
- What changes if you run WasmEdge instead of Wasmtime?
