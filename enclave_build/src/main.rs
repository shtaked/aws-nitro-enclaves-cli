// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use clap::{App, AppSettings, Arg};
use std::fs::OpenOptions;

use enclave_build::Docker2Eif;

fn main() {
    let matches = App::new("Docker2Eif builder")
        .about("Generate consistent EIF image from a Docker image")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("docker_image")
                .short("t")
                .long("tag")
                .help("Docker image tag")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("init_path")
                .short("i")
                .long("init")
                .help("Path to a binary representing the init process for the enclave")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("kernel_img_path")
                .short("k")
                .long("kernel")
                .help("Path to a bzImage linux kernel")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("cmdline")
                .short("c")
                .long("cmdline")
                .help("Cmdline for kernel")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("linuxkit_path")
                .short("l")
                .long("linuxkit")
                .help("Linuxkit executable path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output file for EIF image")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("build")
                .short("b")
                .long("build")
                .help("Build image from Dockerfile")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("pull")
                .short("p")
                .long("pull")
                .help("Pull the Docker image before generating EIF")
                .required(false)
                .conflicts_with("build"),
        )
        .get_matches();

    let docker_image = matches.value_of("docker_image").unwrap();
    let init_path = matches.value_of("init_path").unwrap();
    let kernel_img_path = matches.value_of("kernel_img_path").unwrap();
    let cmdline = matches.value_of("cmdline").unwrap();
    let linuxkit_path = matches.value_of("linuxkit_path").unwrap();
    let output = matches.value_of("output").unwrap();
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output)
        .expect("Failed to create output file");

    let mut img = Docker2Eif::new(
        docker_image.to_string(),
        init_path.to_string(),
        kernel_img_path.to_string(),
        cmdline.to_string(),
        linuxkit_path.to_string(),
        &mut output,
        ".".to_string(),
    )
    .unwrap();

    if matches.is_present("build") {
        let dockerfile_dir = matches.value_of("build").unwrap();
        img.build_docker_image(dockerfile_dir.to_string()).unwrap();
    } else if matches.is_present("pull") {
        img.pull_docker_image().unwrap();
    }

    img.create().unwrap();
}