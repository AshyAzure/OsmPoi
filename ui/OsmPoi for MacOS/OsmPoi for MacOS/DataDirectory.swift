//
//  DataDirectory.swift
//  OsmPoi for MacOS
//
//

import Foundation
import SwiftUI


class DataDirectory: ObservableObject {
    @Published var files: [URL] = []
    
    let url = try! FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true).appendingPathComponent("Datasets")
    
    private lazy var folderMonitor = FolderMonitor(url: self.url)
    
    init() {
        if !FileManager.default.fileExists(atPath: url.path) {
            try! FileManager.default.createDirectory(at: url, withIntermediateDirectories: false, attributes: nil)
        }
        
        folderMonitor.folderDidChange = { [weak self] in
            self?.updateFiles()
        }
        
        folderMonitor.startMonitoring()
        self.updateFiles()
    }
    
    func updateFiles() {
        let files = (try? FileManager.default.contentsOfDirectory(at: url, includingPropertiesForKeys: nil, options: .producesRelativePathURLs)) ?? []
        DispatchQueue.main.async {
            self.files = files
        }
    }
}

class FolderMonitor {
    
    private var monitoredFolderFileDescriptor: CInt = -1
    
    private let folderMonitorQueue = DispatchQueue(label: "FolderMonitorQueue", attributes: .concurrent)
    
    private var folderMonitorSource: DispatchSourceFileSystemObject?
    
    let url: Foundation.URL
    
    var folderDidChange: (() -> Void)?
    
    init(url: Foundation.URL) {
        self.url = url
    }
    
    func startMonitoring() {
        guard folderMonitorSource == nil &&
                monitoredFolderFileDescriptor == -1 else {
            return
        }
        monitoredFolderFileDescriptor = open(url.path, O_EVTONLY)
        folderMonitorSource = DispatchSource.makeFileSystemObjectSource(
            fileDescriptor: monitoredFolderFileDescriptor,
            eventMask: .write,
            queue: folderMonitorQueue)
        folderMonitorSource?.setEventHandler { [weak self] in
            self?.folderDidChange?()
        }
        folderMonitorSource?.setCancelHandler { [weak self] in
            guard let strongSelf = self else { return }
            close(strongSelf.monitoredFolderFileDescriptor)
            strongSelf.monitoredFolderFileDescriptor = -1
            strongSelf.folderMonitorSource = nil
        }
        folderMonitorSource?.resume()
    }
    
    func stopMonitoring() {
        folderMonitorSource?.cancel()
    }
}

extension DataDirectory {
    func createFile() {
        let file = UUID().uuidString
        FileManager.default.createFile(atPath: self.url.appendingPathComponent(file).path, contents: nil, attributes: nil)
    }
    
    func deleteFile(url: URL) {
        try? FileManager.default.removeItem(at: url)
    }
}
