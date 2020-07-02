
        export class Point {
            private readonly _view: DataView;
            constructor(buffer: ArrayBuffer, arrayPtr: number, ix: number) {
const size = 8;

                const offset = ix * size;
                this._view = new DataView(buffer, arrayPtr + offset, size);
            }
            get x(): number {
const offset = 0;

                return this._view.getFloat32(offset, true);
            }
            get y(): number {
const offset = 4;

                return this._view.getFloat32(offset, true);
            }
        }
        