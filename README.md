# SwipeDesign

SwipeDesign is an intuitive, easy-to-use UI design tool leveraging GPT-4 and Rust+WASM for seamless design iterations. It aims to simplify the design process by allowing users to create UI designs with simple left and right swipes, similar to browsing content on TikTok.

## Demo v0.1.0
![image](https://github.com/RevAtN/SwipeDesign/blob/main/v0.1.0.gif)

## Features

- Intuitive user interface with left and right swipe gestures for quick design iterations
- Powered by GPT-4 for generating UI designs based on user requirements
- Rust+WASM for efficient performance and seamless browser interaction
- Simple front-end stack (HTML, CSS, and JavaScript) for easy customization
- Iterative design process with user feedback loops for continuous improvement

## v0.1.0 Features

- Fetch GPT-4 content based on user input
- Update HTML preview with fetched content
- Save and load API key from local storage

## TODO list:

- [x] Fetch GPT-4 content based on user input
- [x] Update HTML preview with fetched content
- [x] Save and load API key from local storage
- [ ] Implement user interface for design iteration with "Satisfied" and "Not Satisfied" buttons
- [ ] Create functions to handle user feedback and modify the generated HTML code accordingly
- [ ] Integrate the design iteration process with GPT-4 to continuously improve the UI design
- [ ] Optimize Rust code for better performance and efficiency
- [ ] Test the application across different browsers and platforms for compatibility
- [ ] Write comprehensive documentation on how to use and extend the application
- [ ] Add support for CSS and JavaScript generation in addition to HTML
- [ ] Implement user authentication and secure API key storage
- [ ] Create a better UI for the application to enhance user experience
- [ ] Consider adding pre-built UI components for users to choose from


## Getting Started


### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Installation

1. Clone the repository:
```git clone https://github.com/RevAtN/SwipeDesign.git```

2. Navigate to the project directory:
```cd SwipeDesign```

3. Build the Rust+WASM package:
```wasm-pack build --target web```

4. Start a local server to test the application
```python3 -m http.server 8000```

## Notes
To reduce testing costs, gpt-3.5-turbo is used by default, but gpt4 is more powerful and can be enabled by modifying the 'model' parameter in src/lib.rs.

## Contributing

We welcome contributions to SwipeDesign! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your changes.
3. Commit your changes and push them to your fork.
4. Open a pull request from your fork to the main repository.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
