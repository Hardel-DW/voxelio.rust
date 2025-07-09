import { NbtFile, NbtStream, NbtAccessor } from '../src/index.js';

export interface BenchmarkResult {
    name: string;
    timeMs: number;
    memoryMB: number;
    operations: number;
    opsPerSecond: number;
}

export class NbtBenchmark {
    private results: BenchmarkResult[] = [];

    async runBenchmarks(testData: Uint8Array, iterations: number = 1000): Promise<BenchmarkResult[]> {
        console.log(`Running NBT benchmarks with ${iterations} iterations...`);

        // Benchmark 1: Basic file reading
        await this.benchmarkFileReading(testData, iterations);

        // Benchmark 2: Path navigation
        await this.benchmarkPathNavigation(testData, iterations);

        // Benchmark 3: Accessor performance
        await this.benchmarkAccessors(testData, iterations);

        // Benchmark 4: Streaming vs regular
        await this.benchmarkStreaming(testData, iterations);

        // Benchmark 5: Batch processing
        await this.benchmarkBatchProcessing(testData, iterations);

        return this.results;
    }

    private async benchmark(
        name: string,
        fn: () => Promise<void> | void,
        iterations: number
    ): Promise<BenchmarkResult> {
        // Warm up
        for (let i = 0; i < Math.min(10, iterations); i++) {
            await fn();
        }

        // Force garbage collection if available
        if (global.gc) {
            global.gc();
        }

        const memBefore = process.memoryUsage();
        const startTime = performance.now();

        for (let i = 0; i < iterations; i++) {
            await fn();
        }

        const endTime = performance.now();
        const memAfter = process.memoryUsage();

        const timeMs = endTime - startTime;
        const memoryMB = (memAfter.heapUsed - memBefore.heapUsed) / 1024 / 1024;
        const opsPerSecond = (iterations / timeMs) * 1000;

        const result: BenchmarkResult = {
            name,
            timeMs,
            memoryMB,
            operations: iterations,
            opsPerSecond
        };

        this.results.push(result);
        console.log(`${name}: ${opsPerSecond.toFixed(2)} ops/s, ${timeMs.toFixed(2)}ms total, ${memoryMB.toFixed(2)}MB`);

        return result;
    }

    private async benchmarkFileReading(testData: Uint8Array, iterations: number): Promise<void> {
        await this.benchmark('File Reading', () => {
            const nbt = NbtFile.read(testData);
            // Access some data to ensure parsing
            nbt.getString('root');
        }, iterations);

        await this.benchmark('Lazy Reading', () => {
            const nbt = NbtFile.readLazy(testData);
            // Access some data to ensure parsing
            nbt.getString('root');
        }, iterations);

        await this.benchmark('Fields Reading', () => {
            const nbt = NbtFile.readFields(testData, ['Data.Player.Name', 'Data.Player.Level']);
            // Access the specific fields
            nbt.getString('Data.Player.Name');
        }, iterations);
    }

    private async benchmarkPathNavigation(testData: Uint8Array, iterations: number): Promise<void> {
        const nbt = NbtFile.read(testData);

        await this.benchmark('String Path Navigation', () => {
            nbt.getString('Data.Player.Level');
        }, iterations);

        await this.benchmark('Array Path Navigation', () => {
            nbt.getString(['Data', 'Player', 'Level']);
        }, iterations);

        // Test complex path
        await this.benchmark('Complex Path Navigation', () => {
            nbt.getString('Data.Player.inventory[0].tag.display.Name');
        }, iterations);
    }

    private async benchmarkAccessors(testData: Uint8Array, iterations: number): Promise<void> {
        const nbt = NbtFile.read(testData);
        const accessor = nbt.createAccessor('Data.Player.Level');

        await this.benchmark('Direct Path Access', () => {
            nbt.getString('Data.Player.Level');
        }, iterations);

        await this.benchmark('Accessor Access', () => {
            accessor.getString();
        }, iterations);

        // Multiple accessors
        const accessors = [
            nbt.createAccessor('Data.Player.Name'),
            nbt.createAccessor('Data.Player.Level'),
            nbt.createAccessor('Data.Player.Health')
        ];

        await this.benchmark('Multiple Accessors', () => {
            accessors.forEach(acc => acc.getString());
        }, iterations);
    }

    private async benchmarkStreaming(testData: Uint8Array, iterations: number): Promise<void> {
        await this.benchmark('Regular Field Access', async () => {
            const nbt = NbtFile.read(testData);
            nbt.getString('Data.Player.Name');
            nbt.getString('Data.Player.Level');
        }, Math.floor(iterations / 10)); // Fewer iterations for async

        await this.benchmark('Streaming Field Access', async () => {
            const stream = await NbtFile.createStream(testData);
            await stream.readField('Data.Player.Name');
            await stream.readField('Data.Player.Level');
            stream.close();
        }, Math.floor(iterations / 10));

        await this.benchmark('Streaming Fields Batch', async () => {
            const fields = await NbtFile.readFieldsStream(testData, [
                'Data.Player.Name',
                'Data.Player.Level'
            ]);
            fields.get('Data.Player.Name');
        }, Math.floor(iterations / 10));
    }

    private async benchmarkBatchProcessing(testData: Uint8Array, iterations: number): Promise<void> {
        const files = Array(10).fill(testData);

        await this.benchmark('Individual Processing', () => {
            files.forEach(fileData => {
                const nbt = NbtFile.read(fileData);
                nbt.getString('Data.Player.Name');
            });
        }, Math.floor(iterations / 10));

        await this.benchmark('Batch Processing', () => {
            NbtFile.processBatch(files, (nbt) => {
                nbt.getString('Data.Player.Name');
            });
        }, Math.floor(iterations / 10));

        await this.benchmark('Batch Reading', () => {
            const nbts = NbtFile.readBatch(files);
            nbts.forEach(nbt => nbt.getString('Data.Player.Name'));
        }, Math.floor(iterations / 10));
    }

    printResults(): void {
        console.log('\n=== NBT Benchmark Results ===');
        console.log('Name\t\t\t\tOps/Sec\t\tTime(ms)\tMemory(MB)');
        console.log('-'.repeat(70));

        for (const result of this.results) {
            const name = result.name.padEnd(30);
            const ops = result.opsPerSecond.toFixed(2).padStart(8);
            const time = result.timeMs.toFixed(2).padStart(8);
            const mem = result.memoryMB.toFixed(2).padStart(8);
            console.log(`${name}\t${ops}\t\t${time}\t\t${mem}`);
        }
    }

    getResults(): BenchmarkResult[] {
        return [...this.results];
    }

    clear(): void {
        this.results = [];
    }
}

// Usage example and factory function
export function createBenchmark(): NbtBenchmark {
    return new NbtBenchmark();
}

export async function runQuickBenchmark(testData: Uint8Array): Promise<BenchmarkResult[]> {
    const benchmark = createBenchmark();
    const results = await benchmark.runBenchmarks(testData, 100);
    benchmark.printResults();
    return results;
} 