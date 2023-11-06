using System;
using System.Runtime.InteropServices;

namespace samick.sample;

internal class Program
{
    public delegate void Callback(int arg);

	[DllImport("libs/libimage_utils_wasm_native.so")]
    private static extern int get_value_i32();
    [DllImport("libs/libimage_utils_wasm_native.so")]
    private static extern float get_value_f32();
    [DllImport("libs/libimage_utils_wasm_native.so")]
    private static extern String pass_string([MarshalAs(UnmanagedType.LPUTF8Str)] string arg);
    [DllImport("libs/libimage_utils_wasm_native.so")]
    private static extern void fn_with_callback(Callback callback);

    static void Main(string[] args)
    {
        Console.WriteLine("get_value_i32 {0}", get_value_i32());
        Console.WriteLine("get_value_f32 {0}", get_value_f32());
        Console.WriteLine("pass_string {0}", pass_string("String from C# Program"));
        Callback callback = (arg) => {
            Console.WriteLine("Callback: {0}", arg);
        };
        fn_with_callback(callback);
    }
}
