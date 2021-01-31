//
//  DataDirectory.swift
//  OsmPoi for MacOS
//
//

import Foundation
import SwiftUI


class DataDirectory: ObservableObject {
    
    @Published var files: [URL] = []
    
    private let createDatabaseQueue = DispatchQueue(label: "CreateDatabaseQueue")
    
    let url = try! FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true).appendingPathComponent("Datasets")
    
    init() {
        if !FileManager.default.fileExists(atPath: url.path) {
            try! FileManager.default.createDirectory(at: url, withIntermediateDirectories: false, attributes: nil)
        }
        self.updateFiles()
    }
    
    func updateFiles() {
        let files = (try? FileManager.default.contentsOfDirectory(at: url, includingPropertiesForKeys: nil, options: .producesRelativePathURLs)) ?? []
        DispatchQueue.main.async {
            self.files = files.filter { url in
                url.pathExtension == "poi"
            }
        }
    }

    func createFile() {
        let panel = NSOpenPanel()
        if panel.runModal() == .OK {
            if let pbf_url = panel.url {
                let db_url = self.url.appendingPathComponent(pbf_url.lastPathComponent).deletingPathExtension().appendingPathExtension("poiparsing")
                createDatabaseQueue.async {
                    print(osmpoi_dump(pbf_url.path, db_url.path))
                    print(osmpoi_parse_ways(db_url.path))
                    print(osmpoi_parse_relations(db_url.path))
                    print(osmpoi_refine(db_url.path))
                    try? FileManager.default.moveItem(at: db_url, to: db_url.deletingPathExtension().appendingPathExtension("poi"))
                    self.updateFiles()
                }
            }
        }
    }
    
    func deleteFile(url: URL) {
        try? FileManager.default.removeItem(at: url)
        self.updateFiles()
    }
}
