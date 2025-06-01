package com.mlt.decoder;

import static org.junit.jupiter.api.Assertions.*;

import com.mlt.data.MapLibreTile;
import com.mlt.metadata.tileset.MltTilesetMetadata;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

public class MltDecoderTest3 {

  @Test
  public void testDecodeOptimizedMltTile() throws IOException {
    System.out.println("Reading tile bytes...");
    Path tilePath =
        Paths.get(
            "/Users/weixingzhang/work/maplibre-tile-spec/ts/test/data/omt/unoptimized/mlt/plain/0_0_0.mlt");
    byte[] tileBytes = Files.readAllBytes(tilePath);
    System.out.println("Tile bytes length: " + tileBytes.length);

    System.out.println("Reading metadata bytes...");
    Path metadataPath =
        Paths.get(
            "/Users/weixingzhang/work/maplibre-tile-spec/ts/test/data/omt/unoptimized/mlt/plain/tileset.pbf");
    byte[] metadataBytes = Files.readAllBytes(metadataPath);
    System.out.println("Metadata bytes length: " + metadataBytes.length);

    System.out.println("Parsing metadata...");
    MltTilesetMetadata.TileSetMetadata metadata =
        MltTilesetMetadata.TileSetMetadata.parseFrom(metadataBytes);

    System.out.println("Decoding tile...");
    MapLibreTile tile = MltDecoder.decodeMlTile(tileBytes, metadata);

    System.out.println("Decoded tile: " + tile);
    assertNotNull(tile, "Decoded tile should not be null");
  }
}
