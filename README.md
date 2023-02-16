# ImClasRegAn
## Image Classification and Regression Annotation Tool

![white box asking user to choose task](https://github.com/cbosoft/imclasregan/blob/master/screenshots/imclasregan_splash.png?raw=true)

A super easy to use, super fast, tool for annotating images by class (for classification ML tasks) or relative to one another (for regression ML tasks).

# Backgroun
I write about my motivations for this [here](https://cmjb.tech/blog/2023/02/09/gamifying-image-annotation/) and I've also written about some of the development challenges [here](https://cmjb.tech/blog/2023/02/10/imclasregan-rs/).

# Installation and usage (server side)
Clone the repo...
```bash
git clone https://github.com/cbosoft/imclasregan
```

Build the server. Ensure you have a [working Rust install](https://www.rust-lang.org/learn/get-started).
```bash
cd imclasregan
cargo build --release
```

No errors? Good. Run the server.
```bash
export IMCLASREGAN_PORT=8765  # choose a port
export IMCLASREGAN_IP=127.0.0.1  # set your IP here
cargo run --release
```

Go to site. You will see output ending with:
```bash
...
🚀 Rocket has launched from http://127.0.0.1:8765
```
(Or with whatever IP/Port combo you used.) Click the link to see the site!

# Usage (client side)

The user is greeted by a splash page asking them to choose a task: classification or regression.

![white box asking user to choose task](https://github.com/cbosoft/imclasregan/blob/master/screenshots/imclasregan_splash.png?raw=true)

Clicking or tapping on the task will bring them to the task page.

![white box asking user to choose task](https://github.com/cbosoft/imclasregan/blob/master/screenshots/imclasregan_classification.png?raw=true)

The classification task page will present an image to the user and ask them to select the option which best describes the image. In the above, we're asking if the image is of a dog, or of a cat. The classes have a short description included, to relieve any ambiguity with one-word labels.

The regression task page works slightly differently. The user is presented with a pair of images, and is asked to choose which one more represents a given *quality*. This is used to form an ordered list, aiding in the development of quantifiable description functions for regression tasks.