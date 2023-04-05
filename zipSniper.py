import requests
import argparse
import struct
import uuid
import os
from tqdm import tqdm

class eocd_struct_64:

    def __init__(self, bytestream):

        self.bytestream = bytestream
        unpacked_stream = struct.unpack("<LQHHLLQQQQ", self.bytestream[:56])
        self.size_of_eocd64 = unpacked_stream[1]
        self.version_made_by = unpacked_stream[2]
        self.min_version_needed = unpacked_stream[3]
        self.disk_number = unpacked_stream[4]
        self.disk_where_cd_starts = unpacked_stream[5]
        self.number_of_cd_records_on_disk = unpacked_stream[6]
        self.total_number_of_cd_records = unpacked_stream[7]
        self.cd_size = unpacked_stream[8]
        self.cd_start_offset = unpacked_stream[9]

class eocd_struct:

    def __init__(self, bytestream):

        self.bytestream = bytestream
        unpacked_stream = struct.unpack("<LHHHHIIH", self.bytestream[:22])
        self.disk_number = unpacked_stream[1]
        self.disk_where_cd_starts = unpacked_stream[2]
        self.number_of_cd_records_on_disk = unpacked_stream[3]
        self.total_number_of_cd_records = unpacked_stream[4]
        self.cd_size = unpacked_stream[5]
        self.cd_start_offset = unpacked_stream[6]

class cd_struct:

    def __init__(self, bytestream):

        self.bytestream = bytestream
        unpacked_stream = struct.unpack("<IHHHHHHIIIHHHHHII", self.bytestream[:46])
        self.version_made_by = unpacked_stream[1]
        self.min_version_needed = unpacked_stream[2]
        self.general_purpose_flag = unpacked_stream[3]
        self.compresion_method = unpacked_stream[4]
        self.last_modification_time = unpacked_stream[5]
        self.last_modification_date = unpacked_stream[6]
        self.crc32_uncompressed_checksum = unpacked_stream[7]
        self.compressed_size = unpacked_stream[8]
        self.uncompressed_size = unpacked_stream[9]
        self.file_name_length = unpacked_stream[10]
        self.extra_field_length = unpacked_stream[11]
        self.file_comment_length = unpacked_stream[12]
        self.disk_number_where_file_starts = unpacked_stream[13]
        self.internal_file_attributes = unpacked_stream[14]
        self.external_file_attributes = unpacked_stream[15]
        self.relative_offset = unpacked_stream[16]
        self.file_name = bytestream[46:46+self.file_name_length]

class zipSniper():

    def __init__(self, remote_path: str, comment_buffer: int, http_proxy, https_proxy):

        self.remote_path = remote_path
        self.directory_list = list()
        self.eocd_blob = None
        self.proxies = dict()
        if http_proxy is not None:
            self.proxies["http"] = http_proxy
        if https_proxy is not None:
            self.proxies["https"] = https_proxy
        if comment_buffer < 0:
            self.comment_buffer = comment_buffer * -1
        else:
            self.comment_buffer = comment_buffer
        self.zip_size = self._zip_size()
        self.is64 = False
        self._eocd_blob()
        self._cd_blob()
        self._sanity_check()

    def _zip_size(self):

        response = requests.head(
            self.remote_path,
            allow_redirects=True,
            proxies=self.proxies
        )

        result = int(response.headers["Content-Length"])
        print(f"ZIP Size: {result}")
        return result

    def _eocd_blob(self):

        response = requests.get(
            self.remote_path,
            allow_redirects=True,
            headers={"Range": f"bytes=-{self.comment_buffer}"},
            proxies=self.proxies
        )

        eocd_sig_64 = b'\x50\x4b\x06\x06'
        eocd_sig = b'\x50\x4b\x05\x06'

        binary_blob = bytearray(response.content)
        for position, byte in enumerate(binary_blob):
            if hex(byte) == "0x50":
                if eocd_sig == binary_blob[position:position+4]:
                    self.is64 = False
                elif eocd_sig_64 == binary_blob[position:position+4]:
                    self.is64 = True
                else:
                    continue
                print(f"Is Zip64: {self.is64}")
                print(f"EOCD Blob offset: {hex(self.zip_size - position)}")
                if self.is64:
                    self.eocd_blob = eocd_struct_64(binary_blob[position:self.zip_size])
                else:
                    self.eocd_blob = eocd_struct(binary_blob[position:self.zip_size])
                return

        raise MissingEocdSig(self.comment_buffer)
    
    def _cd_blob(self):

        cd_offset = self.eocd_blob.cd_start_offset
        cd_size = self.eocd_blob.cd_size

        response = requests.get(
            self.remote_path,
            allow_redirects=True,
            headers={"Range": f"bytes={cd_offset}-{cd_offset + cd_size - 1}"},
            proxies=self.proxies,
            stream=True
        )

        block_size = 1024
        progress_bar = tqdm(total=(cd_size - 1), unit="iB", unit_scale=True)
        binary_blob = bytearray()

        filename = str(uuid.uuid4())
        with open(filename, "wb") as file:
            for d in response.iter_content(block_size):
                progress_bar.update(len(d))
                file.write(d)

        progress_bar.close()

        cd_sig = b'\x50\x4b\x01\x02'
        
        binary_blob = open(filename, "rb").read()
        for position, byte in enumerate(binary_blob):
            if hex(byte) == "0x50":
                if cd_sig == binary_blob[position:position+4]:
                    cd_data = cd_struct(binary_blob[position:-1])
                    self.directory_list.append(cd_data.file_name.decode("utf-8"))
        os.remove(filename)

    def _sanity_check(self):

        if len(self.directory_list) != self.eocd_blob.total_number_of_cd_records:
            raise IncompleteDirectoryList(len(self.directory_list), self.eocd_blob.total_number_of_cd_records)

                
class MissingEocdSig(Exception):

    def __init__(self, comment_buffer, *args):
        self.comment_buffer = comment_buffer
        self.message = f"Comment Buffer of {comment_buffer} is too small!"
        super().__init__(self.message, *args)


class IncompleteDirectoryList(Exception):

    def __init__(self, correct_size, found_size, *args):
        self.correct_size = correct_size
        self.found_size = found_size
        self.message = f"EOCD Reports {correct_size} CD Records. Only {found_size} found!"
        super().__init__(self.message, *args)


if __name__ == "__main__":

    parser = argparse.ArgumentParser(
        prog="zipSniper",
        description="Pulls the directory structure of a zip file remotely",
    )
    parser.add_argument("url", type=str, help="Remote path of the zip archive")
    parser.add_argument("-c", 
                        "--comment_buffer",
                        dest="comment_buffer", 
                        type=int, 
                        default=1024,
                        metavar="int",
                        help="The size of the comment in EOCD is unknown. So to ensure we can find the EOCDs position in the ZIP archive, we specify a random buffer (Default: 1024 Bytes)"
                    )
    parser.add_argument("--http_proxy", dest="http_proxy", type=str, default=None, metavar="url")
    parser.add_argument("--https_proxy", dest="https_proxy", type=str, default=None, metavar="url")
    parser.add_argument("-O", "--output_file", dest="output_file", type=str, default=None)
    args = parser.parse_args()

    sniper = zipSniper(args.url, args.comment_buffer, args.http_proxy, args.https_proxy)
    if args.output_file is not None:
        with open(args.output_file, "w") as file:
            for d in sniper.directory_list:
                file.write(d + "\n")
    else:
        print(sniper.directory_list)
