#![allow(warnings)]

use std::process::Command;
use std::io::{self, Write};

fn display (width: i32, height: i32, buffer: [[i32; 100]; 100]) {
    for y in 0..height as usize{
        for x in 0..width as usize{
            // print!("{}", buffer[y][x]);
            if(buffer[y][x] == 1){
                print!("#");
            }else{
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn bl_put_pixel (x: i32, y: i32, buffer: &mut [[i32; 100]; 100]) {
 buffer[y as usize][x as usize] = 1;
}

fn bl_make_line (x1: i32, y1: i32, x2: i32, y2: i32, buffer: &mut [[i32; 100]; 100]) {
 let mut x = x1;
 let mut y = y1;

 let mut dx = (x2 - x1).abs();
 let mut dy = (y2 - y1).abs();

 let mut sx = 0;
 let mut sy = 0;

 if (x1 < x2) { sx = 1; } else { sx = -1; }
 if (y1 < y2) { sy = 1; } else { sy = -1; }

 let mut err = dx - dy;

 loop {
    //println!("inlooped");
    buffer[y as usize][x as usize] = 1;

    if (x == x2 && y == y2) {
        break;
    }

    let mut e2 = 2 * err;

    if (e2 > -dy) {
        err = err - dy;
        x = x + sx;
    }

    if (e2 < dx) {
        err = err + dx;
        y = y + sy;
    }
 }
}

fn bl_make_triangle (tris_buff: [[i32; 2]; 3], buffer: &mut [[i32; 100]; 100] ) {
    bl_make_line(tris_buff[0][0], tris_buff[0][1], tris_buff[1][0], tris_buff[1][1], buffer);
    bl_make_line(tris_buff[1][0], tris_buff[1][1], tris_buff[2][0], tris_buff[2][1], buffer);
    bl_make_line(tris_buff[2][0], tris_buff[2][1], tris_buff[0][0], tris_buff[0][1], buffer);
}

fn bl_mtx_multiply (vertex: &mut [i32], op_mtx: [[i32; 4]; 4]) {
    let mut res = [0; 4];

    for i in 0..4 as usize {
        for j in 0..4 as usize{
            res[i] += op_mtx[i][j] * vertex[j];
        }
    }

    vertex.copy_from_slice(&res);
}

fn bl_translate_mesh (msh_buff: &mut Vec<Vec<i32>>, trits: i32,op_matrix:[[i32; 4]; 4] ) {
    for t in 0..trits as usize{
        //println!("operate mtx");
        let mut tmp_vertex1 = [ msh_buff[t][0], msh_buff[t][1], msh_buff[t][2], 1]; // v1: [x,y,z, 1] |
        let mut tmp_vertex2 = [ msh_buff[t][3], msh_buff[t][4], msh_buff[t][5], 1]; // v2: [x,y,z, 1] +- Triangle
        let mut tmp_vertex3 = [ msh_buff[t][6], msh_buff[t][7], msh_buff[t][8], 1]; // v3: [x,y,z, 1] |
        //println!("multiply:");
        bl_mtx_multiply(&mut tmp_vertex1, op_matrix);
        bl_mtx_multiply(&mut tmp_vertex2, op_matrix);
        bl_mtx_multiply(&mut tmp_vertex3, op_matrix);
        //println!("write address:");
        let new_tris = [
            tmp_vertex1[0], tmp_vertex1[1], tmp_vertex1[2],
            tmp_vertex2[0], tmp_vertex2[1], tmp_vertex2[2],
            tmp_vertex3[0], tmp_vertex3[1], tmp_vertex3[2]
        ].to_vec();
        //println!("push: {:?}", new_tris);
        msh_buff[t] = new_tris;
    }
}

fn bl_draw_3d_msh (msh_buff: &mut Vec<Vec<i32>>, trits: i32, buffer: &mut [[i32; 100]; 100]) {
    for t in 0..trits as usize{
        //println!("operate mtx");
        let mut tris_buffer: [[i32; 2]; 3] = [
            [ msh_buff[t][0]/msh_buff[t][2], msh_buff[t][1]/msh_buff[t][2]], // v1: [x/z,y/z] |
            [ msh_buff[t][3]/msh_buff[t][5], msh_buff[t][4]/msh_buff[t][5]], // v2: [x/z,y/z] +- Triangle
            [ msh_buff[t][6]/msh_buff[t][8], msh_buff[t][7]/msh_buff[t][8]], // v3: [x/z,y/z] |
        ];

        bl_make_triangle(tris_buffer, buffer);
    }
}

fn main(){
    let mut camera_matrix: [[i32; 4]; 4] = [
        [  1,  0,  0, 25],
        [  0,  1,  0, 10],
        [  0,  0,  1,  1],
        [  0,  0,  0,  1],
    ];

    let mut triangle_buffer: [[i32; 2]; 3] = [
        [ 3, 3],
        [20, 3],
        [20,20],
    ];

    let mut mesh_buffer: Vec<Vec<i32>> = vec![vec![0; 9]; 2]; // unlimited mesh buffer for testing.

    mesh_buffer[0] = vec![ 0,  0,  0, 10, 10,  0,  0, 10,  0];
    mesh_buffer[1] = vec![ 0,  0,  0, 10,  0,  0, 10, 10,  0];

    let mut frame_buffer: [[i32; 100]; 100] = [[0; 100]; 100];
    let mut cmd = String::new();
    let mut fpc = 0; // Frame counter OwO

    bl_translate_mesh(&mut mesh_buffer, 2, camera_matrix);

    while(true) {
        bl_make_line(  0,  0, 49,  0, &mut frame_buffer);
        bl_make_line( 49,  0, 49, 24, &mut frame_buffer);
        bl_make_line( 49, 24,  0, 24, &mut frame_buffer);
        bl_make_line(  0,  0,  0, 24, &mut frame_buffer);
        bl_draw_3d_msh(&mut mesh_buffer, 2, &mut frame_buffer);
        display(50,25, frame_buffer);
        println!("endl");
        println!("Frame cnt: {}", fpc);
        io::stdin().read_line(&mut cmd).expect("line error");
        Command::new("clear").status().expect("Failed to clear screen");
        fpc = fpc + 1;
    }
}
