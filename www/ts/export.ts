
export class PointArray {
    private readonly _view: DataView;
    constructor(buffer: ArrayBuffer, ptr: number, count: number) {
        this._view = new DataView(buffer, ptr, count * 8);
    }
    
    x(ix: number): number {
        return this._view.getFloat32(ix * 8 + 0, true);
    }
    
    y(ix: number): number {
        return this._view.getFloat32(ix * 8 + 4, true);
    }
    
}
