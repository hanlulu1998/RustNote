import rust_mod
import qrcode
import time



if __name__ == '__main__':
    t0 = time.time()
    # rust包调用
    rust_mod.qrcode_img("hello", "./qrcode1.png")
    t1 = time.time()
    # python包调用
    img = qrcode.make("hello")
    img.save("./qrcode2.png")
    t2 = time.time()

    print("Rust time: %fs" % (t1-t0))
    print("Python time: %fs" % (t2-t1))
