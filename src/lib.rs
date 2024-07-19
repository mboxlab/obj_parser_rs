use std::{ f32::consts::PI, ffi::CString };

use rglua::{ interface, prelude::* };

const DEG2RAD: f32 = PI / 180f32;

fn rotate_vec(mut vec: Vector, pitch: f32, yaw: f32, roll: f32) {
    let p: f32 = pitch * DEG2RAD;
    let y: f32 = yaw * DEG2RAD;
    let r: f32 = roll * DEG2RAD;

    let ysin: f32 = y.sin();
    let ycos: f32 = y.cos();
    let psin: f32 = p.sin();
    let pcos: f32 = p.cos();
    let rsin: f32 = r.sin();
    let rcos: f32 = r.cos();
    let psin_rsin: f32 = psin * rsin;
    let psin_rcos: f32 = psin * rcos;

    let (x, y, z) = (vec.x, vec.y, vec.z);

    vec.x =
        x * (ycos * pcos) +
        y * (ycos * psin_rsin - ysin * rcos) +
        z * (ycos * psin_rcos + ysin * rsin);
    vec.y =
        x * (ysin * pcos) +
        y * (ysin * psin_rsin + ycos * rcos) +
        z * (ysin * psin_rcos - ycos * rsin);
    vec.z = x * -psin + y * (pcos * rsin) + z * (pcos * rcos);
}

#[lua_function]
fn parse(l: LuaState) -> Result<i32, interface::Error> {
    let data: &str = rstr!(luaL_checkstring(l, 1));

    let pos_x: f32 = luaL_checknumber(l, 2) as f32;
    let pos_y: f32 = luaL_checknumber(l, 3) as f32;
    let pos_z: f32 = luaL_checknumber(l, 4) as f32;

    let ang_p: f32 = luaL_checknumber(l, 5) as f32;
    let ang_y: f32 = luaL_checknumber(l, 6) as f32;
    let ang_r: f32 = luaL_checknumber(l, 7) as f32;

    let scale_x: f32 = luaL_checknumber(l, 8) as f32;
    let scale_y: f32 = luaL_checknumber(l, 9) as f32;
    let scale_z: f32 = luaL_checknumber(l, 10) as f32;

    let uv_scale: f32 = luaL_checknumber(l, 11) as f32;

    let model: wavefront::Obj = wavefront::Obj::from_reader(data.as_bytes()).unwrap();
    lua_newtable(l);
    for (name, obj) in model.objects() {
        lua_newtable(l);
        {
            let mut index: f64 = 0f64;
            for poly in obj.polygons() {
                for vert in poly.vertices() {
                    index += 1f64;
                    lua_pushnumber(l, index);
                    lua_newtable(l);
                    {
                        let position: [f32; 3] = vert.position();
                        let normal: [f32; 3] = vert.normal().unwrap();
                        let uv: [f32; 3] = vert.uv().unwrap();
                        let pos = Vector::new(
                            position[0] * scale_x + pos_x,
                            position[1] * scale_z + pos_z,
                            position[2] * scale_y + pos_y
                        );

                        rotate_vec(pos, ang_p, ang_y, ang_r);

                        lua_pushvector(l, pos);
                        lua_setfield(l, -2, cstr!("pos"));

                        let normal: Vector = Vector::new(normal[0], normal[1], normal[2]);

                        lua_pushvector(l, normal);
                        lua_setfield(l, -2, cstr!("normal"));

                        lua_pushnumber(l, (uv[0] * uv_scale) as f64);
                        lua_setfield(l, -2, cstr!("u"));

                        lua_pushnumber(l, ((1f32 - uv[1]) * uv_scale) as f64);
                        lua_setfield(l, -2, cstr!("v"));
                    }
                    lua_settable(l, -3);
                }
            }
        }
        let str: CString = CString::new(name)?;
        lua_setfield(l, -2, str.as_ptr());
    }
    Ok(1)
}

#[lua_function]
fn parse_without_edit(l: LuaState) -> Result<i32, interface::Error> {
    let data: &str = rstr!(luaL_checkstring(l, 1));

    match wavefront::Obj::from_reader(data.as_bytes()) {
        Ok(model) => {
            lua_newtable(l);
            for (name, obj) in model.objects() {
                lua_newtable(l);
                {
                    let mut index: f64 = 0f64;
                    for poly in obj.polygons() {
                        for vert in poly.vertices() {
                            index += 1f64;
                            lua_pushnumber(l, index);
                            lua_newtable(l);
                            {
                                let position: [f32; 3] = vert.position();
                                let normal: [f32; 3] = vert.normal().unwrap();
                                let uv: [f32; 3] = vert.uv().unwrap();
                                let pos = Vector::new(position[0], position[1], position[2]);

                                lua_pushvector(l, pos);
                                lua_setfield(l, -2, cstr!("pos"));

                                let normal: Vector = Vector::new(normal[0], normal[1], normal[2]);

                                lua_pushvector(l, normal);
                                lua_setfield(l, -2, cstr!("normal"));

                                lua_pushnumber(l, uv[0] as f64);
                                lua_setfield(l, -2, cstr!("u"));

                                lua_pushnumber(l, (1f32 - uv[1]) as f64);
                                lua_setfield(l, -2, cstr!("v"));
                            }
                            lua_settable(l, -3);
                        }
                    }
                }
                let str: CString = CString::new(name)?;
                lua_setfield(l, -2, str.as_ptr());
            }
        }
        Err(err) => {
            printgm!(l, "{}", err);
        }
    }

    Ok(1)
}

#[gmod_open]
fn main(l: LuaState) -> Result<i32, interface::Error> {
    let lib = reg![
        "parse" => parse,
        "parseWithoutEdit" => parse_without_edit
    ];

    luaL_register(l, cstr!("ObjParserRS"), lib.as_ptr());
    Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
    0
}
