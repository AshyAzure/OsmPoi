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
    public sealed partial class QueryPage : Page
    {

        private ObservableCollection<Dataset> datasets = new ObservableCollection<Dataset>();

        public ObservableCollection<Dataset> Datasets { get { return datasets; } }

        public QueryPage()
        {
            this.InitializeComponent();
            updateDatasets();
        }

        private async void updateDatasets()
        {
            var folder = ApplicationData.Current.LocalCacheFolder;
            var new_datasets = new ObservableCollection<Dataset>();
            foreach (var file in await folder.GetFilesAsync())
            {
                new_datasets.Add(new Dataset(file.Path));
            }
            datasets = new_datasets;
        }

        private string InputButtonName
        { 
            get
            {
                /// todo!
                return "choose input";
            } 

        }

        private string OutputButtonName
        {
            get
            {
                /// todo!
                return "choose output";
            }

        }

        private void InputButton_Click(object sender, RoutedEventArgs e)
        {

        }

        private void OutputButton_Click(object sender, RoutedEventArgs e)
        {

        }

        private void QueryButton_Click(object sender, RoutedEventArgs e)
        {

        }
    }

}
