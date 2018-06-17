//
//  ViewController.swift
//  audiosnap
//
//  Created by Antti Lamminsalo on 15/06/2018.
//  Copyright © 2018 Antti Lamminsalo. All rights reserved.
//

import Cocoa

class ViewController: NSViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        print("Load : libaudiosnap", String(cString: c_version()))
    }

    override var representedObject: Any? {
        didSet {
        // Update the view, if already loaded.
        }
    }


}

