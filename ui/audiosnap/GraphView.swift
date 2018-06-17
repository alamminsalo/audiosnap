//
//  GraphView.swift
//  audiosnap
//
//  Created by Antti Lamminsalo on 15/06/2018.
//  Copyright © 2018 Antti Lamminsalo. All rights reserved.
//

import Cocoa

class GraphView: NSView {
    
    struct GraphState {
        var data: [Int16]
        var splitpoints: [UInt32]
        var zoom: Double
        var ceil: Int32
    }
    
    var state: GraphState = GraphState(data: [], splitpoints: [], zoom: 1.0, ceil: INT16_MAX)

    override func draw(_ dirtyRect: NSRect) {
        super.draw(dirtyRect)
        
        // white bg
        //NSColor.white.setFill()
        //bounds.fill()
        
        let context = NSGraphicsContext.current?.cgContext
        drawGraph(context: context)
        drawSplits(context: context)
        drawCeil(context: context)
    }
    
    func drawGraph(context: CGContext?) {
        let path = CGMutablePath()
        
        path.move(to: CGPoint(x: 0, y: self.bounds.height / 2))
        
        let h = Double(self.bounds.height)
        let w = state.zoom / Double(state.data.count) * Double(bounds.width)
        let imax = Double(INT16_MAX)
        
        print(state.data.count)
        
        for (index, val) in state.data.enumerated() {
            let x: Double = Double(index) * w
            let y: Double = Double(val) / imax * h + h / 2;
            
            path.addLine(to: CGPoint(x: x, y: y))
        }
        path.closeSubpath()
        
        context?.setLineWidth(0.2)
        context?.setStrokeColor(CGColor.init(red: 0.0, green: 0.1, blue: 1.0, alpha: 1.0))
        context?.setShouldAntialias(false)
        
        context?.addPath(path)
        context?.drawPath(using: .stroke)
    }
    
    func drawCeil(context: CGContext?) {
        let path = CGMutablePath()
        
        let a = Double(self.bounds.height) / 2
        let b = Double(state.ceil) / Double(INT16_MAX)
        let c = a + (a * b)
        print(a,b,c)
        path.move(to: CGPoint(x: 0, y: c))
        path.addLine(to: CGPoint(x: Double(self.bounds.width), y: c))
        path.closeSubpath()
        
        context?.setLineWidth(0.3)
        context?.setStrokeColor(CGColor(red: 0, green: 1, blue: 0, alpha: 1))
        context?.setShouldAntialias(false)
        
        context?.addPath(path)
        context?.drawPath(using: .stroke)
    }
    
    func drawSplits(context: CGContext?) {
        let path = CGMutablePath()
        
        let h = Double(self.bounds.height)
        let w = state.zoom / Double(state.data.count) * Double(bounds.width)
        
        for index in state.splitpoints {
            let x: Double = Double(index) * w
            
            path.move(to: CGPoint(x: x, y: 0))
            path.addLine(to: CGPoint(x: x, y: h))
            path.closeSubpath()
        }
        
        context?.setLineWidth(0.8)
        context?.setStrokeColor(CGColor(red: 1, green: 0, blue: 0, alpha: 1))
        context?.setShouldAntialias(false)
        
        context?.addPath(path)
        context?.drawPath(using: .stroke)
    }
    
    @IBAction func loadButtonClicked(button: NSButton)
    {
        let dialog = NSOpenPanel();
        
        dialog.title                   = "Choose a .wav file";
        dialog.showsResizeIndicator    = false;
        dialog.showsHiddenFiles        = false;
        dialog.canChooseDirectories    = false;
        dialog.canCreateDirectories    = false;
        dialog.allowsMultipleSelection = false;
        dialog.allowedFileTypes        = ["wav"];
        
        if (dialog.runModal() == NSApplication.ModalResponse.OK) {
            let result = dialog.url // Pathname of the file
            if result != nil
            {
                let path = result!.path
                loadFile(path: path)
            }
        }
    }
    
    @IBAction func ceilSliderChanged(slider: NSSlider)
    {
        state.ceil = slider.intValue
        getSplits()
    }
    
    // Loads given file
    func loadFile(path: String)
    {
        let cpath = path.cString(using: String.Encoding.utf8)
        let len = c_load_file(cpath)
        print("Loading", path, "(", len, " samples )...")
        
        let cbuf = UnsafeMutablePointer<Int16>.allocate(capacity: len)
        c_data(cbuf, len)
        
        state.data = Array(UnsafeBufferPointer(start: cbuf, count: len))
        print("Loaded.")
        setNeedsDisplay(self.bounds)
    }
    
    func getSplits() {
        let len = c_split(Int16(state.ceil))
        let cbuf = UnsafeMutablePointer<UInt32>.allocate(capacity: len)
        c_splits(cbuf,len)
        state.splitpoints = Array(UnsafeBufferPointer(start: cbuf, count: len))
        setNeedsDisplay(self.bounds)
    }
}
