use std::{mem::size_of, ffi::c_void, marker::PhantomData};
use super::{vertex_buffer::VertexBuffer, index_buffer::IndexBuffer};

pub struct VertexArray<T>
{
    renderer_id : u32, // vao ID
    _vbo : VertexBuffer<T>,
    _ebo: IndexBuffer,
    _phantom: PhantomData<T>
}

impl<T> VertexArray<T>
{
    pub fn new(vertex_buffer: VertexBuffer<T>, vertex_layout: &VertexBufferLayout, index_buffer: IndexBuffer) -> Self
    {
        let mut vao = 0;
        unsafe
        {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        // add the buffer, binds the vertex buffer implicitely
        VertexArray::add_buffer(&vertex_buffer, vertex_layout);
        // bind the index buffer
        index_buffer.bind();
        // unbind the vao
        unsafe { gl::BindVertexArray(0); } 
        
        Self{ _phantom: PhantomData, renderer_id:vao, _vbo:vertex_buffer, _ebo:index_buffer}
    }

    //TODO: Document
    fn add_buffer(vertex_buffer: &VertexBuffer<T> , layout: &VertexBufferLayout)
    {
        // setup
        vertex_buffer.bind();
        let mut offset: usize = 0;
        let mut attrib_index = 0;

        for element in &layout.elements
        {
            unsafe
            {
                if element.integral // if the attribute is of integral type, another API call must be used 
                {
                    gl::VertexAttribIPointer(attrib_index, element.count as _, element.element_type,
                        layout.stride_bytes.try_into().unwrap(), offset as *const c_void);
                }
                else
                {
                    gl::VertexAttribPointer(attrib_index, element.count as _ , element.element_type,
                        element.normalized, layout.stride_bytes.try_into().unwrap()  , offset as *const c_void );
                }
        
                gl::EnableVertexAttribArray(attrib_index);
            }
            attrib_index += 1;
            offset += element.size_bytes;
        };
    }

    pub fn bind(&self)
    {
        unsafe
        {
            gl::BindVertexArray(self.renderer_id);
        }
    }

    pub fn unbind()
    {
        unsafe
        {
            gl::BindVertexArray(0);
        }
    }


}

struct VertexBufferLayoutElement
{
    element_type: u32,
    count: usize,
    normalized: u8,
    size_bytes: usize,
    integral: bool,
}

pub struct VertexBufferLayout
{
    elements: Vec<VertexBufferLayoutElement>,
    stride_bytes: usize,
}

impl VertexBufferLayout
{
    pub fn new() -> Self
    {
        Self{elements: Vec::new(),stride_bytes:0}
    }

    pub fn push_f32(&mut self, count: usize)
    {
        let element = VertexBufferLayoutElement { element_type: gl::FLOAT, count , normalized: gl::FALSE , size_bytes: size_of::<f32>() * count, integral: false};
        self.push_element(element);
    }

    pub fn push_u8(&mut self, count: usize)
    {
        let element = VertexBufferLayoutElement { element_type: gl::UNSIGNED_BYTE, count , normalized: gl::FALSE , size_bytes: size_of::<u8>() * count, integral: true};
        self.push_element(element);
    }

    pub fn _push_unsigned(&mut self, count: usize)
    {
        let element = VertexBufferLayoutElement { element_type: gl::UNSIGNED_INT, count , normalized: gl::FALSE , size_bytes: size_of::<u32>() * count, integral: true};
        self.push_element(element);
    }

    fn push_element(&mut self, element: VertexBufferLayoutElement)
    {
        self.stride_bytes += element.size_bytes;
        self.elements.push(element);
    }
}