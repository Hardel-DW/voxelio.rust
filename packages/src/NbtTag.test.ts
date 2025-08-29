import { describe, it, expect } from 'vitest';
import { NbtFile, NbtTag, NbtType } from './index.js';

describe('NbtTag', () => {
  it('should work with tag from NbtFile', async () => {
    // Create NBT data with compound
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    expect(tag).toBeDefined();
    expect(tag.getType()).toBe(NbtType.Compound);
  });

  it('should handle type checking', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    expect(tag.getType()).toBe(NbtType.Compound);
  });

  it('should convert to string and number', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    
    // Test string conversion
    const stringRep = tag.asString();
    expect(stringRep).toBeDefined();
    expect(typeof stringRep).toBe('string');
    
    // Test number conversion  
    const numberRep = tag.asNumber();
    expect(typeof numberRep).toBe('number');
  });

  it('should handle compound operations', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    
    // Get compound keys
    const keys = tag.getCompoundKeys();
    expect(keys).toContain('foo');
    expect(keys.length).toBeGreaterThan(0);
    
    // Get compound value
    using value = tag.getCompoundValue('foo');
    expect(value).toBeDefined();
    expect(value.getType()).toBe(NbtType.String);
    
    // Test direct access methods
    expect(tag.getString('foo')).toBe('Hello!');
  });

  it('should handle setting values', async () => {
    const data = new Uint8Array([10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    
    // Test setting string
    tag.setString('newKey', 'newValue');
    expect(tag.getString('newKey')).toBe('newValue');
    
    // Test setting number
    tag.setNumber('level', 42);
    expect(tag.getNumber('level')).toBe(42);
  });

  it('should handle lists', async () => {
    // Create NBT with a list
    const data = new Uint8Array([10, 0, 0, 9, 0, 4, 116, 101, 115, 116, 8, 0, 0, 0, 2, 0, 5, 104, 101, 108, 108, 111, 0, 5, 119, 111, 114, 108, 100, 0]);
    using file = await NbtFile.from(data);
    
    using tag = file.getRoot();
    using testList = tag.getCompoundValue('test');
    
    if (testList.getType() === NbtType.List) {
      const length = testList.getListLength();
      expect(length).toBeGreaterThan(0);
      
      // Get list item
      using firstItem = testList.getListItem(0);
      expect(firstItem).toBeDefined();
    }
  });
});