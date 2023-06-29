Reference to [Former Tutorial](https://github.com/aik2mlj/raytracer-tutorial)

### Track 1: Reduce Contention 

此项工作的前提条件是完成多线程渲染。

在多线程环境中，`clone` / `drop Arc` 可能会导致性能下降。因此，我们要尽量减少 `Arc` 的使用。这项任务的目标是，仅在线程创建的时候 `clone Arc`。其他地方不出现 `Arc`，将 `Arc` 改为引用。

### Track 2: Static Dispatch

调用 `Box<dyn trait>` / `Arc<dyn trait>` / `&dyn trait` 中的函数时会产生额外的开销。我们可以通过泛型来解决这个问题。

这个任务的目标是，通过定义新的泛型材质、变换和物体，比如 `LambertianStatic<T>`，并在场景中使用他们，从而减少动态调用的开销。你也可以另开一个模块定义和之前的材质同名的 `struct`。

仅在 `HitRecord`, `ScatterRecord` (这个在第三本书的剩余部分中出现), `HittableList` 和 `BVHNode` 中使用 `dyn`。
如果感兴趣，可以探索如何使用 `macro_rules` 来减少几乎相同的代码写两遍的冗余。

### Track 3: Code Generation 

此项工作的前提条件是完成 `BVH`。

目前，`BVHNode` 是在运行时构造的。这个过程其实可以在编译期完成。我们可以通过过程宏生成所有的物体，并构造静态的 `BVHNode`，从而提升渲染效率。

`codegen` 部分不需要通过 `clippy`。

如果感兴趣，你也可以探索给过程宏传参的方法。e.g. 通过 `make_spheres_impl! { 100 }` 生成可以产生 100 个球的函数。

### Track 4: PDF Static Dispatch

此项工作的前提条件是完成第三本书的剩余部分。PDF 中需要处理的物体使用泛型完成，去除代码路径中的 `&dyn`。


### Track 5: More Code Generation

在过程宏中，读取文件，直接从 `yaml` 或 `JSON` 文件（选择一种即可）生成场景对应的程序。

在 data 文件夹中给出了一些例子。

例子中 `BVHNode` 里的 `bounding_box` 是冗余数据。你可以不使用这个数据。

读 `JSON` / `yaml` 可以调包。

### Track 6: Advanced Features 增加对 Transform 的 PDF 支持。

