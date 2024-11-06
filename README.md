# Ethshadow: Discrete-event Ethereum network simulator

<!--- ANCHOR: overview (for mdbook) -->

**Ethshadow** is a tool for simulating the Ethereum network using an easily manageable configuration. Under the hood, it uses [Shadow](https://shadow.github.io/),
a [discrete-event network simulator](https://en.wikipedia.org/wiki/Discrete-event_simulation), which allows running simulations with real Ethereum clients,
without the need for modifications or specifically written simulation code.

The advantages of using Ethshadow are as follows:

1. **Batteries included**: It includes everything needed for the simulation (e.g., `libp2p`, `discv5`, etc.).
2. **Real-World simulation**: Unlike [monkey patching](https://en.wikipedia.org/wiki/Monkey_patch), Ethshadow uses unmodified clients for simulation, ensuring that the simulation is identical to the mainnet and public testnets.
3. **Client upgrade friendly**: New version of a [supported client](#supported-clients) can be simulated generally without requiring significant changes to the configuration.

To simulate the Ethereum protocol, simply configure any [supported EL and CL clients](#supported-clients), such as Lighthouse and Geth.

<!--- ANCHOR_END: overview (for mdbook) -->

## Quickstart

We assume that you already have Go, Rust, and Docker installed.

Install Lighthouse and Geth.

```sh
# Lighthouse
sudo apt update && sudo apt install -y git gcc g++ make cmake pkg-config llvm-dev libclang-dev clang
git clone https://github.com/sigp/lighthouse.git
cd lighthouse
git checkout v5.3.0 # The latest tested version
make
make install-lcli

# Geth
git clone https://github.com/ethereum/go-ethereum.git
cd go-ethereum
git checkout v1.14.11 # The latest tested version
make all
sudo cp build/bin/geth /usr/local/bin/geth # Make it globally accessible
sudo cp build/bin/bootnode /usr/local/bin/bootnode # Make it globally accessible
```

Install Shadow

```sh
sudo apt-get install -y cmake findutils libclang-dev libc-dbg libglib2.0-0 libglib2.0-dev make netbase python3 python3-networkx xz-utils util-linux gcc g++
git clone https://github.com/shadow/shadow.git
cd shadow
./setup build --clean
./setup install
echo 'export PATH="${PATH}:/home/${USER}/.local/bin"' >> ~/.bashrc && source ~/.bashrc
```

Install Ethshadow.

```sh
git clone https://github.com/ethereum/ethshadow.git
cd ethshadow
cargo install --path .
```

Save the following file to a config file `myfirstsim.yaml`.

```yaml
general:
  # How much time should we simulate?
  stop_time: 10 min
  # Display a progress indicator?
  progress: true

ethereum:
  # Distribute this many validators evenly across all nodes
  validators: 30
  # Create this many nodes with Geth, Lighthouse and a Validator client.
  # Additionally, a host with one boot node for CL and EL each is added.
  nodes: 10
```

Run the simulation.

```sh
ethshadow myfirstsim.yaml
```

Check out `./data/shadow/hosts` which contains the `stdout` and `stderr` of every process (including geth and lighthouse)
of every node.

## Supported clients

<!--- ANCHOR: supported-clients (for mdbook) -->

✅ = Available, works out-of-the-box with latest release

🚧 = Available, works with modifications (see subpage for details)

❌ = Unavailable, does not currently work

❔ = Unavailable, not yet tested

A client is considered to supported if it can follow the chain and perform the necessary duties for validating. Other
features might not work.

### Execution Layer

| Name                         | Node | Boot Node | Latest tested version |
| ---------------------------- | :--: | :-------: | :-------------------: |
| Besu                         |  ❔  |    ❔     |                       |
| Erigon                       |  ❔  |    ❔     |                       |
| EthereumJS                   |  ❔  |    ❔     |                       |
| [Geth](docs/clients/geth.md) |  ✅  |    ✅     |       v1.14.11        |
| Nethermind                   |  ❔  |    ❔     |                       |
| Reth                         |  🚧  |    ❔     |                       |

### Consensus Layer

| Name                                     | Node | Boot Node | Validator Client | Latest tested version |
| ---------------------------------------- | :--: | :-------: | :--------------: | :-------------------: |
| Grandine                                 |  ❔  |    ❔     |        ❔        |                       |
| [Lighthouse](docs/clients/lighthouse.md) |  ✅  |    ✅     |        ✅        |        v5.3.0         |
| Lodestar                                 |  ❔  |    ❔     |        ❔        |                       |
| Nimbus                                   |  ❔  |    ❔     |        ❔        |                       |
| Prysm                                    |  ❔  |    ❔     |        ❔        |                       |
| Teku                                     |  ❔  |    ❔     |        ❔        |                       |

<!--- ANCHOR_END: supported-clients (for mdbook) -->

## More Information

See the [Ethshadow documentation](https://ethereum.github.io/ethshadow) for more information.
