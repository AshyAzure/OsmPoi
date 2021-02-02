using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices.WindowsRuntime;
using Windows.Foundation;
using Windows.Foundation.Collections;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;
using Windows.UI.Xaml.Controls.Primitives;
using Windows.UI.Xaml.Data;
using Windows.UI.Xaml.Input;
using Windows.UI.Xaml.Media;
using Windows.UI.Xaml.Navigation;
using System.Collections.ObjectModel;
using Windows.Storage;

// The Blank Page item template is documented at https://go.microsoft.com/fwlink/?LinkId=234238

namespace OsmPoi_for_Windows
{
    /// <summary>
    /// An empty page that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class DatasetPage : Page
    {
        public DatasetPage()
        {
            this.InitializeComponent();
            updateDatasets();
        }

        private async void AppBarButton_Click(object sender, RoutedEventArgs e)
        {
            var button = sender as AppBarButton;
            if (button != null)
            {
                switch (button.Tag.ToString()) {
                    case "Add":
                        var picker = new Windows.Storage.Pickers.FileOpenPicker();
                        Windows.Storage.StorageFile file = await picker.PickSingleFileAsync();
                        if (file != null)
                        {
                            // todo
                        }
                        updateDatasets();
                        break;
                    case "Delete":
                        var folder = ApplicationData.Current.LocalCacheFolder;

                        updateDatasets();
                        break;
                }
            }
            
        }

        private ObservableCollection<Dataset> datasets = new ObservableCollection<Dataset>();

        public ObservableCollection<Dataset> Datasets { get { return datasets; } }

        private async void updateDatasets()
        {
            var folder = ApplicationData.Current.LocalCacheFolder;
            var new_datasets = new ObservableCollection<Dataset>();
            foreach(var file in await folder.GetFilesAsync())
            {
                new_datasets.Add(new Dataset(file.Path));
            }
            datasets = new_datasets;
        }
    }

    public class Dataset
    {
        public Dataset(string path)
        {
            FilePath = path;
        }

        public string FilePath { get; set; } 
    }
}
