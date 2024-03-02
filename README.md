>This project is under development.

# RedRuntime

## Introduction

***RedRuntime*** is a runtime environment that can run and accelerate redstone circuits. To achieve that, it first convert circuits to [Directed Weighted Graph](https://en.wikipedia.org/wiki/Directed_graph), and then create a runtime to process these graphs instead of Minecraft.

RedRuntime is developed in Rust, which combines fast and memory safety together, as well as provides programmers the guarantee that the written programs are nearly always "correct".

## Architecture

RedRuntime are organized as this architecture:
| Layer  | Module                                                                                 |
| ------ | -------------------------------------------------------------------------------------- |
| top    | Pre-process Module (Including discarding unpowered blocks and sorting the rest blocks) |
|        | Graph Generation Module                                                                |
|        | Unreachable Branch Culling Module                                                      |
| bottom | Runtime                                                                                |

## Principle

All block metadata were pushed into a vector, which then will be processed by RedRuntime later.

- To accelerate this process, multi threading will be introduced in RedRuntime. Using multi threading means Redruntime can more effectively to handle and sort blocks. At the same time, however, more bugs will be produced, which will decreate the robustness of RedRuntime. Please switch back to single threading and open an issue if any relavant bugs encounter. (I will provide its API)

RedRuntime will roughly process vector ahead of converting circuits to graphs when vector passed into RedRuntime.

- If such these unpoweredable blocks, like the air block, were fetched out, they will be discarded.
- If redstone wires were fetched out, they will be pushed into a vector, which stores the information of wires.

## License

This software is distuibuted under MIT license.