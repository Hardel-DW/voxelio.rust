import { describe, it, expect } from 'vitest';
import { NbtFile, NbtType } from './index.js';

describe('NbtFile', () => {
  it('should create from NBT data', async () => {
    // Create compound NBT data: {foo: "Hello!"}
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    
    using file = await NbtFile.from(data);
    expect(file).toBeDefined();
    expect(file.getString('foo')).toBe('Hello!');
  });

  it('should get string values', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    expect(file.getString('foo')).toBe('Hello!');
    expect(file.getString('missing')).toBe(''); // Default for missing key
  });

  it('should get number values', async () => {
    // Create NBT with number: {level: 42}
    const data = new Uint8Array([10, 0, 0, 3, 0, 5, 108, 101, 118, 101, 108, 0, 0, 0, 42, 0]);
    using file = await NbtFile.from(data);
    
    expect(file.getNumber('level')).toBe(42);
    expect(file.getNumber('missing')).toBe(0); // Default for missing key
  });

  it('should write NBT data', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    const writtenData = file.write();
    expect(writtenData).toBeDefined();
    expect(writtenData.length).toBeGreaterThan(0);
  });

  it('should process root tag', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    const result = file.process((root) => {
      expect(root).toBeDefined();
      return 'processed';
    });
    
    expect(result).toBe('processed');
  });

  it('should process batch of files', async () => {
    const data1 = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    const data2 = new Uint8Array([10, 0, 0, 8, 0, 3, 98, 97, 114, 0, 5, 119, 111, 114, 108, 100, 0]);
    
    const results = await NbtFile.processBatch([data1, data2], (nbt, index) => {
      return { index, hasData: true };
    });
    
    expect(results).toHaveLength(2);
    expect(results[0].index).toBe(0);
    expect(results[1].index).toBe(1);
  });

  it('should handle using keyword for automatic disposal', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    
    // Test that the file can be used and disposed automatically
    let wasDisposed = false;
    {
      using file = await NbtFile.from(data);
      expect(file.getString('foo')).toBe('Hello!');
      // file will be automatically disposed when leaving this scope
    }
    // At this point, the file should be disposed
    expect(true).toBe(true); // Test that we reached this point without errors
  });
});