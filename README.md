>This project is under development.

# RedRuntime

## Introduction

***RedRuntime*** is a runtime environment that can run and accelerate redstone circuits. To achieve that, it first convert circuits to [Directed Weighted Graph](https://en.wikipedia.org/wiki/Directed_graph), and then create a runtime to process these graphs instead of Minecraft.

## Principle

All block information were pushed into a vector, which then can be processed by RedRuntime later.

- To accelerate this process, multi-thread will be introduced in RedRuntime. Using multi-thread means that more bugs will be produced and that will decreate the robustness of RedRuntime. Please switch back to single-thread and open an issue if any relavant bugs encounter. (I will provide its API)

RedRuntime will roughly process vector ahead of converting circuits to graphs.

- If such these unpoweredable blocks - like the air block - were fetched out, they will be discarded.
- If redstone wires were fetched out, they will be pushed into a vector, which stores the information of wires.

## License

This software is distuibuted under MIT license.